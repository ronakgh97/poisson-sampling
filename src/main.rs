mod plot;

use crate::plot::plotter;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
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
    let total_points = 10000;

    let plane = Arc::new(Mutex::new(PLane2D::new(2500.0, 2500.0)));
    let progress = Arc::new(Mutex::new(ProgressBar::new(total_points as u64)));

    // Setup progress bar style
    {
        let prog = progress.lock().unwrap();
        prog.set_style(
            ProgressStyle::default_bar()
                .template("[{bar:40.cyan/blue}] {pos}/{len} ({percent}%) ({elapsed_precise})")
                .unwrap()
                .progress_chars("##>-"),
        );
    }

    // SMALLER RADIUS to fit more points!
    let point_radius = 100.0;

    println!("Starting Poisson sampling...");

    (0..total_points * 10000).into_par_iter().for_each(|_| {
        let plane_clone = Arc::clone(&plane);
        let progress_clone = Arc::clone(&progress);

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

            // Update progress bar (needs its own lock)
            progress_clone.lock().unwrap().inc(1);
        }
    });

    // Finish progress bar
    progress.lock().unwrap().finish();

    let final_plane = plane.lock().unwrap();
    let point_count = final_plane.points.len();

    let output_path = format!(
        "output/poisson_sampling_{}points_{}radius.png",
        point_count, point_radius
    );

    // Get a &PLane2D from the MutexGuard for the plotter function
    match plotter(&*final_plane, &output_path) {
        Ok(_) => println!("Plot saved to {}", output_path),
        Err(e) => eprintln!("Error generating plot: {}", e),
    }

    println!(
        "Generated {} points with minimum distance: {}",
        point_count, point_radius
    );
}
