use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Debug, Clone)]
struct Data{
    pub classes: Vec<u8>,
    pub features: Vec<[f32;13]>
}

fn main() {
    let data= read_wine_data().expect("Unable to read the data");
    println!("{:?}",data);
}


/// Function to read the wine data from the file
/// The data is stored in the resources folder
/// 
/// # Returns
/// 
fn read_wine_data() -> Result<Data, Box<dyn std::error::Error>> { //Result type to handle diffrent kind of errors     
    let mut classes:Vec<u8> = Vec::new(); //vector to store the classes
    let mut features:Vec<[f32;13]> = Vec::new(); //vector to store the features this set has only 13
    let file = File::open("resources/wine.data")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let values= line.split(",").collect::<Vec<&str>>();
        let class = values[0].trim().parse::<u8>()?;
        classes.push(class);
    let ft = values[1..].iter().map(|x| x.trim().parse::<f32>()).collect::<Result<Vec<f32>, _>>()?;
    let feature_array: [f32; 13] = ft.try_into().map_err(|_| "Failed to convert feature vector to array")?;
    features.push(feature_array);
    }

    Ok(Data{classes,features})
}
