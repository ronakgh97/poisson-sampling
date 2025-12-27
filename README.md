# Poisson-Sampling

This repository contains an implementation of the Poisson Disk Sampling algorithm in Rust. The algorithm generates a
set of points that are evenly distributed within a specified area, ensuring that no two points are closer than a given
minimum distance (radius). This technique is commonly used in computer graphics, procedural generation, and spatial
sampling.

## Brute force Plot Samples

- Points: 1803
- Radius: 100
- Area: 5000 x 5000

![Sampling 1803_100_5000x5000](output/poisson_sampling_1803points_100radius_idbe65f9b5-fb87-486e-b8e2-292598bc96b5.png)

- Points: 1500
- Radius: 100
- Area: 5000 x 5000

![Sampling 1500_100_5000x5000](output/poisson_sampling_1500points_100radius_id334de575-0a8d-4e0f-8d45-62640799e895.png)

- Points: 4007
- Radius: 100
- Area: 7500 x 7500

![Sampling 4007_100_7500x7500](output/poisson_sampling_4007points_100radius_id20d15cbe-0999-4ef3-b91f-fe3e385bdbee.png)