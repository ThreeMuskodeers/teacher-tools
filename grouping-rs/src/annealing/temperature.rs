pub fn change_temp(max_temp: f64, min_temp: f64, steps: i32, step: i32) -> anyhow::Result<f64> {
    linear_change_temp(max_temp, min_temp, steps, step)
}

fn linear_change_temp(max_temp: f64, min_temp: f64, steps: i32, step: i32) -> anyhow::Result<f64> {
    if steps == 0 {
        return Err(anyhow::anyhow!("steps must be greater than 0"));
    }
    let delta = max_temp - min_temp;
    Ok(max_temp - (delta * step as f64) / steps as f64)
}
