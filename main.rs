
// DS210 Final Project

extern crate csv;

use plotly::common::Mode;
use plotly::{Layout, Plot, Scatter};

use polyfit_rs as poly;


#[derive(Debug)]


// data structure for price & sqft

struct DataFrame {
   	price: Vec<f64>,
   	sqft: Vec<f64>,
}


// implementation of dataframe struct

impl DataFrame {

    fn new() -> DataFrame {
        DataFrame {
            price: Vec::new(),
            sqft: Vec::new(),
        }
     }
     
     fn read_csv(filepath: &str, has_headers: bool) -> DataFrame {
     
         // Open file
         let file = std::fs::File::open(filepath).unwrap();
         let mut rdr = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(file);
         let mut data_frame = DataFrame::new();
         
         // push all the records
         for result in rdr.records().skip(1).into_iter() {
            let record = result.unwrap();
            data_frame.push(&record);
         }
         return data_frame;
      }
      
      fn push(&mut self, row: &csv::StringRecord) {
          self.price.push(row[0].parse().unwrap()); 
          self.sqft.push(row[1].parse().unwrap()); 
      }
      
}


fn main() {

	// read csv file
   	let d = DataFrame::read_csv("sf_clean.csv", false);
   	
   	
   	// scatter plot
	let trace1 = Scatter::new(d.sqft, d.price)
        .name("trace1")
        .mode(Mode::Markers);
        
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    
    let layout = Layout::new().title("<b>Sq ft vs Price</b>".into());
    plot.set_layout(layout);
    
    plot.show();
    
    
    // linear regression
    
    let data1 = DataFrame::read_csv("sf_clean.csv", false);
    
    let xs = data1.sqft;
    let ys = data1.price;

	let fitted_parameters = poly::polyfit_rs::polyfit(&xs, &ys, 1).unwrap();
	
	
	// test if error > 0
	
	let mut i = 0;
	
    for x in xs {
        let fitted_y: f64 = fitted_parameters[0] + fitted_parameters[1] * x;
        let error = (fitted_y - ys[i]).abs();
        assert!(error >= 0.0);
        i += 1;
    }
    
    // print slope & intercept
    
    println!("Slope:{:?}", fitted_parameters[1]);
    println!("Intercept:{:?}", fitted_parameters[0]);
    
}