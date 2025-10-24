//! # Calculus Modülü
//!
//! Bu modül, temel matematiksel işlemleri gerçekleştiren fonksiyonlar içerir.
//! Örnek olarak, türev ve integral hesaplamaları için fonksiyonlar sağlar.
//!
//! # Örnekler
//! ```rust
//! mod calculus;
//!
//! use calculus::{derivative, integral};
//! fn main() {
//!   let f = |x: f64| x.powi(2);
//!   let deriv_at_3 = derivative(f, 3.0, 1e-7);
//!   println!("f'(3) yaklaşık olarak: {}", deriv_at_3); // Yaklaşık 6.0
//!   let integral_result = integral(f, 0.0, 1.0, 1000);
//!   println!("∫f(x)dx from 0 to 1 yaklaşık olarak: {}", integral_result); // Yaklaşık 0.3333
//! }
//! ```

pub mod calculus;
