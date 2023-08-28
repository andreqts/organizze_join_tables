use std::{error::Error, fs };
use csv::StringRecord;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "Andre Quites Ordovas Santos <andrequites@gmail.com>")]
#[command(version = "0.1.0")]

struct Args {
    /// Output file name
    #[arg(long)]
    output: Option<String>,
}

fn list_files_in_dir(fpath: &str) -> Vec<String> {
    let mut vfiles: Vec<String> = Vec::new();
    let dirv = fs::read_dir(fpath);
    let dirv = match dirv {
        Ok(direc) => direc,
        Err(error) => panic!("Error opening dir '{}': '{:?}'", fpath, error),
    };
    for f in dirv {
        let f = match f {
            Ok(file) => file,
            Err(error) => panic!("Error reading file name in folder '{}': '{:?}'", fpath, error),
        };
        vfiles.push(f.path().display().to_string());
    }
    vfiles
}

const REC_SIZE: usize = 6;
fn append_csv_table_from_file(filepath: &str, dataset: &mut Vec<StringRecord>) -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_path(filepath)?;


    for result in rdr.records() {
        let rec = match result {
            Ok(record) => record,
            Err(error) => {
                let err_desc = format!("Error: invalid file in '{}' - {error}", filepath).to_string();
                return Err(err_desc.into());
            }
        };

        if rec.len() != REC_SIZE {
            let err_desc = format!("Error: invalid record size = {} - expected {REC_SIZE}", rec.len())
                .to_string();
            return Err(err_desc.into()); 
        }
        dataset.push(rec);
    }
    
    Ok(())
}

fn write_dataset_to_csv_file(filepath: &str, dataset: &Vec<StringRecord>) -> Result<(), Box<dyn Error>> {
    let wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .quote_style(csv::QuoteStyle::Never)
        .from_path(&filepath);
    assert!(wtr.is_ok());
    let mut wtr = wtr.unwrap();

    let headerstr = vec!["Data","Descrição","Categoria","Valor","Situação","Informações adicionais\r"]; //TODOAQ: verificar...
    let res = wtr.write_record(headerstr);
    assert!(res.is_ok());
    for row in dataset.iter() {
        let mut rowstr: Vec<String> = Vec::new();
        for (i, rec) in row.iter().enumerate() {
            let mut recstr = rec.to_string();
            if cfg!(windows) && i == (REC_SIZE - 1) {
                recstr = recstr.clone() + "\r"; //add CRLF ending for windows
            }
            rowstr.push(recstr);
        }

        wtr.write_record(rowstr).expect("Error writing csv row in file {filepath}");
    }

    let r = wtr.flush();
    assert!(r.is_ok());
    Ok(())
}

fn main() {
    let cli = Args::parse();

    let outfile = match cli.output {
        Some(f) => f,
        None => String::from("output.csv"),
    };

    println!("Using output file = '{}'", outfile);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_dirs() {    
        let vf  = list_files_in_dir(".\\tests\\testdir");
        let expected_results = [ "f1.txt", "f2.txt", "f3.txt" ];
        for (i, f) in vf.iter().enumerate() {
            let expected_file = expected_results[i];
            assert!(f.contains(expected_file));
        }
    }

    //https://docs.rs/csv/latest/csv/
    #[test]
    fn test_read_csv() {
        let mut dataset : Vec<StringRecord> = Vec::new();
        let res1 = append_csv_table_from_file(".\\tests\\testcsv\\test.csv", &mut dataset);
        assert!(res1.is_ok());
        assert_eq!(dataset.len(), 3);
    }

    #[test]
    fn test_read_invalid_dir() {
        let mut dataset : Vec<StringRecord> = Vec::new();
        let res2 = append_csv_table_from_file(".\\tests\\testcsv_invalid\\test.csv", &mut dataset);
        assert!(res2.is_err());
    }

    #[test]
    fn test_read_fake_csv() {
        let mut dataset : Vec<StringRecord> = Vec::new();
        let res2 = append_csv_table_from_file(".\\tests\\testcsv\\test - missing 1 col.csv", &mut dataset);
        assert!(res2.is_err());
    }

    #[test]
    fn test_csv_table_append() {
        let expected = vec![
            StringRecord::from(vec!["05.04.2023", "Descricao despesa 1", "Transporte", "-65,66", "Não pago",""]),
            StringRecord::from(vec!["05.03.2023", "Livro técnico", "Educação", "-421,66", "Não pago", ""]),
            StringRecord::from(vec!["05.02.2023", "Aluguel", "Moradia", "66", "Pago", ""]),
            StringRecord::from(vec!["05.07.2023", "Descricao despesa 3", "Saúde", "-1200,66", "Não pago", ""]), 
            StringRecord::from(vec!["05.06.2023", "Curso UDEMY", "Educação", "-27,99", "Não pago", ""]),
            StringRecord::from(vec!["05.05.2023", "NETFLIX", "Lazer", "24,90", "Pago", ""])
        ];
        let mut dataset : Vec<StringRecord> = Vec::new();
        let res = append_csv_table_from_file(".\\tests\\testcsv\\test.csv", &mut dataset);
        assert!(res.is_ok());
        assert_eq!(dataset.len(), 3);
        let res = append_csv_table_from_file(".\\tests\\testcsv\\test2.csv", &mut dataset);
        assert!(res.is_ok());
        assert_eq!(dataset.len(), expected.len());
        for it in dataset.iter().zip(expected.iter()) {
            let (data_read, data_exp) = it;
            assert_eq!(data_read, data_exp);
        }
    }
    
    #[test]
    fn test_csv_table_write() {
        let expected = vec![
            StringRecord::from(vec!["05.04.2023", "Descricao despesa 1", "Transporte", "-65,66", "Não pago",""]),
            StringRecord::from(vec!["05.03.2023", "Livro técnico", "Educação", "-421,66", "Não pago", ""]),
            StringRecord::from(vec!["05.02.2023", "Aluguel", "Moradia", "66", "Pago", ""]),
            StringRecord::from(vec!["05.07.2023", "Descricao despesa 3", "Saúde", "-1200,66", "Não pago", ""]), 
            StringRecord::from(vec!["05.06.2023", "Curso UDEMY", "Educação", "-27,99", "Não pago", ""]),
            StringRecord::from(vec!["05.05.2023", "NETFLIX", "Lazer", "24,90", "Pago", ""])
        ];
        
        let testdir: &str = ".\\tests\\testcsv";
        let filename: &str = "joined.csv";
        let joinedfilepath: String = format!("{}\\{}", testdir, filename);

        // delete from previous tests if exist
        let _ = fs::remove_file(&joinedfilepath);

        let res = write_dataset_to_csv_file(&joinedfilepath, &expected);
        assert!(res.is_ok());

        let mut dataset : Vec<StringRecord> = Vec::new();
        let res = append_csv_table_from_file(&joinedfilepath, &mut dataset);
        assert!(res.is_ok());
        assert_eq!(dataset.len(), expected.len());

        for it in dataset.iter().zip(expected.iter()) {
            let (data_read, data_exp) = it;
            assert_eq!(data_read, data_exp);
        }


    }

}