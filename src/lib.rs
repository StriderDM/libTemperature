use std::convert::From;
use std::fmt;
use std::cmp::{PartialEq};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
pub struct Kelvin(f64);

impl fmt::Display for Kelvin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for Kelvin {
    fn eq(&self, other: &Self) -> bool {
         self.0 == other.0
    }
}

impl PartialOrd for Kelvin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<f64> for Kelvin
{
        fn from(item: f64) -> Self
        {
            if item >= 0f64.into()
            {
                Kelvin(item)
            }
            else
            {
                Kelvin(0f64)
            }
        }
}

impl From<Kelvin> for f64
{
        fn from(item: Kelvin) -> f64
        {
            item.0.into()
        }
}

impl From<Celcius> for Kelvin
{
        fn from(item: Celcius) -> Self
        {
            Kelvin ((f64::from(item.0) + 273f64).into())
        }
}

impl From<Fahrenheit> for Kelvin
{
        fn from(item: Fahrenheit) -> Self
        {
            Kelvin ((((5f64/9f64)*(f64::from(item.0)-32f64)) + 273f64).into())
        }
}

#[derive(Debug, Copy, Clone)]
pub struct Fahrenheit(Kelvin);

impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°", self.0)
    }
}

impl PartialEq for Fahrenheit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Fahrenheit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<f64> for Fahrenheit
{
        fn from(item: f64) -> Self
        {
            Fahrenheit(item.into())
        }
}

impl From<Fahrenheit> for f64
{
        fn from(item: Fahrenheit) -> f64
        {
            item.0.into()
        }
}

impl From<Kelvin> for Fahrenheit
{
        fn from(item: Kelvin) -> Self
        {
            Fahrenheit((((9f64/5f64)*(f64::from(item.0)-273f64)) + 32f64).into())
        }
}

impl From<Celcius> for Fahrenheit
{
        fn from(item: Celcius) -> Self
        {
           Fahrenheit((((9f64/5f64)*(f64::from(item.0)))+32f64).into())
        }
}

#[derive(Debug, Copy, Clone)]
pub struct Celcius(Kelvin);

impl fmt::Display for Celcius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}°", self.0)
    }
}

impl PartialEq for Celcius {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for Celcius {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl From<f64> for Celcius
{
        fn from(item: f64) -> Self
        {
            Celcius(item.into())
        }
}

impl From<Celcius> for f64
{
        fn from(item: Celcius) -> f64
        {
            item.0.into()
        }
}

impl From<Kelvin> for Celcius
{
        fn from(item: Kelvin) -> Self
        {
            Celcius((f64::from(item.0) - 273f64).into())
        }
}

impl From<Fahrenheit> for Celcius
{
        fn from(item: Fahrenheit) -> Self
        {
            Celcius(((5f64/9f64)*(f64::from(item.0)-32f64)).into())
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
     fn kelvin_f64() {
        let m = Kelvin::from(300f64);
        assert!(f64::from(m)==300f64);
     }

    #[test]
     fn celcius_f64() {
        let m = Celcius::from(300f64);
        assert!(f64::from(m)==300f64);
     }

    #[test]
     fn fahrenheit_f64() {
        let m = Fahrenheit::from(300f64);
        assert!(f64::from(m)==300f64);
     }

    #[test]
      fn convert() {
        let m = Celcius::from(0f64);
        let n = Kelvin::from(m);
        let o = Fahrenheit::from(m);
        assert!(n==m.into() && o == m.into() && n == o.into()); // celcius == fahrenheit == kelvin
      }

    #[test]
      fn kelvin_not_below_absolute_zero() {
        let m = Kelvin::from(-1000f64);
        assert!(0f64<=m.into());
      }

    #[test]
      fn fahrenheit_not_below_absolute_zero() {
        let m = Fahrenheit::from(-1000f64);
        assert!(0f64<=m.into());
      }

    #[test]
     fn celcius_not_below_absolute_zero() {
        let m = Celcius::from(-1000f64);
        assert!(0f64<=m.into());
     }
}
