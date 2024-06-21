use std::path::Path;

pub fn execpress (xml_value: String, rows_num: u64) -> (u32, String) {
     let mut errcode: u32 = 0;
     let mut errstring: String = "all good and now process execution".to_string();
     let targetfullname = format!("{}__tmpcvs", xml_value);
     if !Path::new(&targetfullname).exists() {
         if Path::new(&xml_value).exists() {
             if rows_num < 10 {
                 errcode = 3;
                 errstring = "The number of rows is less than 10".to_string();
             }
         } else {
             errstring = "the xml file does not exist".to_string();
             errcode = 4;
         }
     } else {
         errstring = format!("the output file exists:{}",targetfullname);
         errcode = 5;
     }
     (errcode, errstring)
}

