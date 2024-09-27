use std::fs::File;
use std::io::{BufRead, BufReader};
use statrs::statistics::Statistics;
use statrs::distribution::Normal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("data/samples/one_day_sample.txt")?;
    let reader = BufReader::new(file);

    let mut data: Vec<Vec<f64>> = Vec::new();

    // Skip the first line (header)
    for line in reader.lines().skip(1) {
        let line = line?;
        let values: Vec<f64> = line.split(',')
            .skip(3) // Skip the first 3 columns
            .map(|s| s.parse().unwrap_or(0.0))
            .collect();
        data.push(values);
    }

    // Transpose the data for column-wise analysis
    let columns: Vec<Vec<f64>> = (0..data[0].len())
        .map(|i| data.iter().map(|row| row[i]).collect())
        .collect();

    println!("Statistical Analysis:");

    for (i, column) in columns.iter().enumerate() {
        let mean = column.mean();
        let std_dev = column.std_dev();
        
        println!("Column {}: Mean = {:.2}, Std Dev = {:.2}", i + 1, mean, std_dev);

        // Check for anomalies (values more than 3 standard deviations from the mean)
        let anomalies: Vec<(usize, f64)> = column.iter().enumerate()
            .filter(|&(_, &value)| (value - mean).abs() > 3.0 * std_dev)
            .map(|(index, &value)| (index, value))
            .collect();

        if !anomalies.is_empty() {
            println!("  Anomalies detected: {:?}", anomalies);
        }

        // Simple trend detection using moving average
        let window_size = 10;
        let moving_averages: Vec<f64> = column.windows(window_size)
            .map(|window| window.iter().sum::<f64>() / window_size as f64)
            .collect();

        if moving_averages.len() > 2 {
            let trend = moving_averages.last().unwrap() - moving_averages.first().unwrap();
            if trend.abs() > std_dev {
                println!("  Possible trend detected: {:.2}", trend);
            }
        }

        // Basic autocorrelation to check for cycles
        let lag = 10;
        let autocorr = autocorrelation(column, lag);
        if autocorr.abs() > 0.5 {
            println!("  Possible cyclical pattern detected (lag {}): {:.2}", lag, autocorr);
        }

        println!();
    }

    Ok(())
}

fn autocorrelation(data: &[f64], lag: usize) -> f64 {
    let n = data.len();
    let mean = data.mean();
    
    let numerator: f64 = data.iter().skip(lag).zip(data.iter())
        .map(|(&x1, &x2)| (x1 - mean) * (x2 - mean))
        .sum();
    
    let denominator: f64 = data.iter()
        .map(|&x| (x - mean).powi(2))
        .sum();
    
    numerator / denominator
}