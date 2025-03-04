pub struct Calculator;

impl Calculator {
    pub fn sum(&self, numbers: &Vec<i32>) -> i32 {
        numbers.iter().sum()
    }
    pub fn avg(&self, numbers: &Vec<i32>) -> f32 {
        let sum = self.sum(numbers) as f32;
        let count = numbers.len() as f32;
        sum / count
    }
    pub fn max(&self, numbers: &Vec<i32>) -> i32 {
        panic!("Not implemented yet");
    }
    pub fn min(&self,numbers: &Vec<i32>) -> i32{
        panic!("Not implemented yet");
    }
}
