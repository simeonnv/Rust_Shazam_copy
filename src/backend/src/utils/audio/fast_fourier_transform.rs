use num_complex::Complex;
use std::f64::consts::PI;

pub fn fast_fourier_transform(input: Vec<f64>) -> Vec<Complex<f64>> {
    let mut complex_array: Vec<Complex<f64>> =
        input.iter().map(|&v| Complex::new(v, 0.0)).collect();

    recursive_fft(&mut complex_array);
    complex_array
}

fn recursive_fft(data: &mut Vec<Complex<f64>>) {
    let n = data.len();
    if n <= 1 {
        return;
    }

    let mut even: Vec<Complex<f64>> = Vec::with_capacity(n / 2);
    let mut odd: Vec<Complex<f64>> = Vec::with_capacity(n / 2);
    for i in 0..n / 2 {
        even.push(data[2 * i]);
        odd.push(data[2 * i + 1]);
    }

    recursive_fft(&mut even);
    recursive_fft(&mut odd);

    for k in 0..n / 2 {
        let theta = -2.0 * PI * (k as f64) / (n as f64);
        let t = Complex::new(theta.cos(), theta.sin()) * odd[k];
        data[k] = even[k] + t;
        data[k + n / 2] = even[k] - t;
    }
}
