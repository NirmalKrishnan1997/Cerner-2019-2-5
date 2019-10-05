/******************************************************************************

 Rust Program for cerner_2^5_2019

 *******************************************************************************/
use std::collections::HashMap;
fn main() {
  let  mut numbers = vec![42, 1, 36, 34, 76, 378, 43, 1, 43, 54, 2, 3, 43];

  let avg: f32;
  let median: i32;
  let mode: i32;

  { // calculate average
      let mut sum: i32 = 0;
      for x in &numbers {
          sum = sum + x;
      }

      avg = sum as f32 / numbers.len() as f32;
  }

  { // calculate median
      numbers.sort();
      let mid = numbers.len() / 2;

      median = numbers[mid];
  }
  { // calculate mode
      // new HashMap
      let mut times = HashMap::new();

      // count
      for x in &numbers {
          let cnt = times.entry(*x as usize).or_insert(0);
          *cnt += 1;
      }

      let mut best: (i32, i32) = (*times.iter().nth(0).expect("Fatal.").0 as i32, *times.iter().nth(0).expect("Fatal.").1 as i32);

      for x in times.iter() {
          if *x.1 > best.1 {
              best = (*x.0 as i32, *x.1);
          }
      }
      mode = best.0;
  }

  println!("AVERAGE: {}", avg);
  println!("MEDIAN: {}", median);
  println!("MODE: {}", mode)

}
