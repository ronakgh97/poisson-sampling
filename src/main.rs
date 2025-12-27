mod plot;

use crate::plot::plotter;
use colored::Colorize;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct PLane2D {
    pub width: f64,
    pub height: f64,
    pub points: Vec<Point2D>,
}

impl PLane2D {
    pub fn new(width: f64, height: f64) -> PLane2D {
        PLane2D {
            width,
            height,
            points: Vec::new(),
        }
    }

    pub fn calculate_expected_points(&self, r: f64) -> usize {
        let area = self.width * self.height;
        let point_area = std::f64::consts::PI * r * r;
        (area / point_area) as usize
    }
}

#[derive(Debug, Clone)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Point2D { x, y, r }
    }

    pub fn new_random(r: f64, plane: &PLane2D) -> Self {
        let mut rng = rand::rng();

        let random_x = rng.random_range(0.0..plane.width);
        let random_y = rng.random_range(0.0..plane.height);

        Point2D {
            x: random_x,
            y: random_y,
            r,
        }
    }

    pub fn try_plot(&self, plane: &PLane2D) -> bool {
        // Check if this point is at least r distance from all existing points
        let too_close_points = plane
            .points
            .iter()
            .filter(|p| {
                let dist_sq = (self.x - p.x).powi(2) + (self.y - p.y).powi(2);
                // Points must be at least r distance apart
                dist_sq < self.r * self.r
            })
            .count();

        // Can plot only if no points are too close
        too_close_points == 0
    }
}

fn main() {
    let total_points = 2000;
    let point_radius = 100.0;
    let uuid = uuid::Uuid::new_v4();

    let plane = Arc::new(Mutex::new(PLane2D::new(10000.0, 10000.0)));
    let max_iterations = total_points * 100000;

    {
        let plan_guard = plane.lock().unwrap();
        let expected = plan_guard.calculate_expected_points(point_radius);
        if expected < total_points {
            eprintln!(
                "Warning: The expected number of points ({}) is less than the requested total points ({}). Consider reducing the point radius or increasing the plane size.",
                expected, total_points
            );
        }
    }

    println!("Starting Poisson sampling...");

    // Setup MultiProgress for displaying both bars
    let multi_progress = Arc::new(MultiProgress::new());

    let points_bar = multi_progress.add(ProgressBar::new(total_points as u64));
    points_bar.set_style(
        ProgressStyle::default_bar()
            .template("Points:     [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("●●>-"),
    );

    let iter_bar = multi_progress.add(ProgressBar::new(max_iterations as u64));
    iter_bar.set_style(
        ProgressStyle::default_bar()
            .template("Iterations: [{bar:40.green/yellow}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("▬▬>-"),
    );

    let progress = Arc::new(points_bar);
    let iteration_progress = Arc::new(iter_bar);
    let iteration_counter = Arc::new(AtomicUsize::new(0));

    (0..max_iterations).into_par_iter().for_each(|_| {
        let plane_clone = Arc::clone(&plane);
        let progress_clone = Arc::clone(&progress);
        let iter_progress_clone = Arc::clone(&iteration_progress);
        let counter_clone = Arc::clone(&iteration_counter);

        // Increment iteration counter and update progress bar every 10000 iterations
        let current_iter = counter_clone.fetch_add(1, Ordering::Relaxed);
        if current_iter % 10000 == 0 {
            iter_progress_clone.set_position(current_iter as u64);
        }

        // Generate a random candidate point
        let candidate = {
            let plane_guard = plane_clone.lock().unwrap();
            Point2D::new_random(point_radius, &plane_guard)
        };

        // Now try to add the candidate point
        let mut plane_guard = plane_clone.lock().unwrap();

        // Stop if we already have enough points
        if plane_guard.points.len() >= total_points {
            return; // Exit this iteration early
        }

        // Check if candidate is valid
        if candidate.try_plot(&plane_guard) {
            plane_guard.points.push(candidate);
            drop(plane_guard); // Explicitly release lock before progress update

            // Update progress bar
            progress_clone.inc(1);
        }
    });

    // Finish progress bars
    let final_iter_count = iteration_counter.load(Ordering::Relaxed);
    iteration_progress.finish_with_message(format!("Completed {} iterations", final_iter_count));
    progress.finish_with_message("Done");

    let final_plane = plane.lock().unwrap();
    let point_count = final_plane.points.len();

    let output_path = format!(
        "output/poisson_sampling_{}points_{}radius_id{}.png",
        point_count, point_radius, uuid
    );

    // Get a &PLane2D from the MutexGuard for the plotter function
    match plotter(&*final_plane, &output_path) {
        Ok(_) => println!("Plot saved to {}", output_path.blue().bold()),
        Err(e) => eprintln!("Error generating plot: {}", e),
    }

    println!(
        "Generated {} points with minimum distance: {}",
        point_count.to_string().green(),
        point_radius.to_string().green()
    );
}
