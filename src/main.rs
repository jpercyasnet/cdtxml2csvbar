use iced::widget::{button, column, row, text, Space};
use iced::{Alignment, Element, Command, Application, Settings, Color, Size};
use iced::theme::{self, Theme};
use iced::executor;
use iced::window;
use std::path::Path;
use std::io::{Write, BufRead, BufReader};
use std::fs::File;
use std::time::Duration as timeDuration;
use std::thread::sleep;

mod get_winsize;
mod inputpress;
mod execpress;
use get_winsize::get_winsize;
use inputpress::inputpress;
use execpress::execpress;

pub fn main() -> iced::Result {

     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32- 75.0;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }

     Xmlparse::run(Settings {
        window: window::Settings {
            size: Size::new(widthxx, heightxx),
            ..window::Settings::default()
        },
        ..Settings::default()
     })
}

struct Xmlparse {
    xml_value: String,
    mess_color: Color,
    msg_value: String,
    rows_num: u64,
}

#[derive(Debug, Clone)]
enum Message {
    XmlPressed,
    ExecPressed,
    ExecxFound(Result<Execx, Error>),
}

impl Application for Xmlparse {
    type Message = Message;
    type Theme = Theme;
    type Flags = ();
    type Executor = executor::Default;
    fn new(_flags: Self::Flags) -> (Xmlparse, iced::Command<Message>) {
        ( Self { xml_value: "--".to_string(), msg_value: "no message".to_string(),
               rows_num: 0, mess_color: Color::from([0.0, 1.0, 0.0]), 
          },
          Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("CDtree xml to vertical bar separated csv for input into sqlite3 -- iced")
    }

    fn update(&mut self, message: Message) -> Command<Message>  {
        match message {
            Message::XmlPressed => {
               let inputstr: String = self.xml_value.clone();
               let (errcode, errstr, newinput) = inputpress(inputstr);
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   if Path::new(&newinput).exists() {
                       self.mess_color = Color::from([0.0, 1.0, 0.0]);
                       self.xml_value = newinput.to_string();
                       self.rows_num = 0;
                       let mut bolok = true;
                       let file = File::open(newinput).unwrap();
                       let mut reader = BufReader::new(file);
                       let mut line = String::new();
                       let mut linenum: u64 = 0;
                       loop {
                          match reader.read_line(&mut line) {
                             Ok(bytes_read) => {
                                 // EOF: save last file address to restart from this address for next run
                                 if bytes_read == 0 {
                                     break;
                                 }
                                 linenum = linenum + 1;
                             }
                             Err(err) => {
                                 self.msg_value = format!("error reading xml {:?} ", err);
                                 self.mess_color = Color::from([1.0, 0.0, 0.0]);
                                 bolok = false;   
                                 break;
                             }
                          };
                       }
                       if bolok {
                           self.rows_num = linenum;
                           self.mess_color = Color::from([0.0, 1.0, 0.0]);
                           self.msg_value = "got xml file and retrieved its number of rows".to_string();
                       } 
                   } else {
                       self.mess_color = Color::from([1.0, 0.0, 0.0]);
                       self.msg_value = format!("xml file does not exist: {}", newinput);
                   }
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Command::none()
           }
            Message::ExecPressed => {
               let (errcode, errstr) = execpress(self.xml_value.clone(), self.rows_num.clone());
               self.msg_value = errstr.to_string();
               if errcode == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
                   Command::perform(Execx::execit(self.xml_value.clone(), self.rows_num.clone()), Message::ExecxFound)
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
                   Command::none()
               }
            }
            Message::ExecxFound(Ok(exx)) => {
               self.msg_value = exx.errval.clone();
               if exx.errcd == 0 {
                   self.mess_color = Color::from([0.0, 1.0, 0.0]);
               } else {
                   self.mess_color = Color::from([1.0, 0.0, 0.0]);
               }
               Command::none()
            }
            Message::ExecxFound(Err(_error)) => {
               self.msg_value = "error in copyx copyit routine".to_string();
               self.mess_color = Color::from([1.0, 0.0, 0.0]);
               Command::none()
            }

        }
    }

    fn view(&self) -> Element<Message> {
        column![
            row![text("Message:").size(20),
                 text(&self.msg_value).size(30).style(*&self.mess_color),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![button("xml input file Button").on_press(Message::XmlPressed).style(theme::Button::Secondary),
                 text(&self.xml_value).size(20).width(1000)
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![text(format!("number of rows: {}", self.rows_num)).size(20), Space::with_width(100),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![Space::with_width(200),
                 button("Exec Button").on_press(Message::ExecPressed),
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![text("if invalid xml need to run:     iconv -f utf-16le -t UTF-8 cdtreeexportallyyyymmdd.xml -o convt.xml").size(15)
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![text("output file is cdtreeexportallyyyymmdd.xml__tmpcvs").size(15)
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![text("run to create new database:     sqlite3 test.db3 < import.sql 2> testout.txt").size(15)
            ].align_items(Alignment::Center).spacing(10).padding(10),
            row![text("import.sql: \n
CREATE TABLE blubackup ( \n
            refname   TEXT NOT NULL, \n
            filename  TEXT NOT NULL, \n
            dirname  TEXT NOT NULL, \n
            filesize INTEGER NOT NULL, \n
            filedate TEXT NOT NULL, \n
            md5sum TEXT, \n
            PRIMARY KEY (refname, filename, dirname, filesize)); \n
.separator | \n
.import cdtreeexportallyyyymmdd.xml__tmpcvs blubackup").size(15)
            ].align_items(Alignment::Center).spacing(10).padding(10),
         ]
        .padding(10)
        .align_items(Alignment::Start)
        .into()
    }

    fn theme(&self) -> Theme {
       Theme::Dark
/*           Theme::custom(theme::Palette {
                        background: Color::from_rgb8(240, 240, 240),
                        text: Color::BLACK,
                        primary: Color::from_rgb8(230, 230, 230),
                        success: Color::from_rgb(0.0, 1.0, 0.0),
                        danger: Color::from_rgb(1.0, 0.0, 0.0),
                    })
*/               
    }
}

#[derive(Debug, Clone)]
struct Execx {
    errcd: u32,
    errval: String,
}

impl Execx {
//    const TOTAL: u16 = 807;

    async fn execit(xml_value: String, rows_num: u64) -> Result<Execx, Error> {
     let mut errstring  = "test of exec ".to_string();
     let mut errcode: u32 = 0;
     let numrows: u64 = rows_num;
     let targetfullname = format!("{}__tmpcvs", xml_value);
     let targeterrname = format!("{}__tmperr", xml_value);
     let file = File::open(xml_value).unwrap(); 
     let mut reader = BufReader::new(file);
     let mut targetfile = File::create(targetfullname.clone()).unwrap();
     let mut targeterr = File::create(targeterrname.clone()).unwrap();
     let mut line = String::new();
     let mut linenum = 0;
     let mut numfiles: u64 = 0;
     let mut slevel = " ";
     let mut scd = String::new();
     let mut sdir = String::new();
     let mut sfile: String;
     let mut sdate = String::new();
     let mut ssize = format!("");
     let mut nameval: String = " ".to_string();
     loop {
         match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                 // EOF: save last file address to restart from this address for next run
                 if bytes_read == 0 {
                     break;
                 }
                 linenum = linenum + 1;
                 if line.contains("<Cd>") {
                     slevel = "Cd";
                 } else if line.contains("<Directory>") {
                     slevel = "Dir";
                 } else if line.contains("<File>") {
                     slevel = "File";
                 } else if line.contains("</File>") {
               	     sfile = nameval.to_string();
                     let mut datechgok = false;
                     if sdate.len() > 8 {
                         let vecdatetime: Vec<&str> = sdate.split(" ").collect();
                         if vecdatetime.len() == 3 {
                             let vecdate: Vec<&str> = vecdatetime[0].split("/").collect();
                             if vecdate.len() == 3 {
                                 let vectime: Vec<&str> = vecdatetime[1].split(":").collect();
                                 if vectime.len() == 3 {
                                     let mo_int: i32 = vecdate[0].parse().unwrap_or(-9999);
                                     if !(mo_int == -9999) {
                                         let da_int: i32 = vecdate[1].parse().unwrap_or(-9999); 
                                         if !(da_int == -9999) {
                                             let yr_int: i32 = vecdate[2].parse().unwrap_or(-9999); 
                                             if !(yr_int == -9999) {
                                                 let mut hr_int: i32 = vectime[0].parse().unwrap_or(-9999); 
                                                 if !(hr_int == -9999) {
                                                     let mi_int: i32 = vectime[1].parse().unwrap_or(-9999);
                                                     if !(mi_int == -9999) {
                                                         let se_int: i32 = vectime[2].parse().unwrap_or(-9999); 
                                                         if !(se_int == -9999) {
                                                             if vecdatetime[2] == "PM" {
                                                                 if hr_int < 12 {
                                                                     hr_int = hr_int + 12;
                                                                 }
                                                                 datechgok = true;
                                                             } else {
                                                                 if vecdatetime[2] == "AM" {
                                                                     if hr_int == 12 {
                                                                         hr_int = 0;
                                                                     }
                                                                     datechgok = true;
                                                                 }
                                                             }
                                                             if datechgok {
                                                                 sdate = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}.000", yr_int, mo_int, da_int, hr_int, mi_int, se_int);
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                     }
                     if !datechgok {
                         let strerr = format!("not valid date saved for: {} {} {} {} {}", scd, sfile, sdir, ssize, sdate);
                         writeln!(&mut targeterr, "{}", strerr).unwrap();
                     }
                     let stroutput = format!("{}|{}|{}|{}|{}",
                                             scd,
                                             sfile,
                                             sdir,
                                             ssize,
                                             sdate);
                     writeln!(&mut targetfile, "{}", stroutput).unwrap();
                     numfiles = numfiles + 1;
                     errstring  = format!("number of files : {}",numfiles);
                     slevel = "Dir";
                 } else if line.contains("<Name>") {
                     let mut lcurrpos = line.find("<Name>").unwrap();
                     let lcurrpos1 = line.find("</Name>").unwrap();
                	 let llen = lcurrpos1 - lcurrpos - 6;
         			 lcurrpos = lcurrpos + 6;
         		     let nameval;
         			 if (lcurrpos1 != 0) & (llen > 0) {
         			     nameval = line.get(lcurrpos..(lcurrpos+llen)).unwrap();
         			 } else {
         			     nameval = "***no /Name or null value***";
         			 }                       
            		 if slevel == "Cd" {
            		     scd = nameval.to_string();
            		 } else if slevel == "Dir" {
         			     if (lcurrpos1 != 0) & (llen > 0) {
         			         sdir = nameval.to_string();
         			     } else {
         			         sdir = "/".to_string();
         				 }                       
            	     } else if slevel == "File" {
                         sdate = format!("");
                         ssize = format!("");
                     }
                 } else if line.contains("<FullName>") {
                     let mut lcurrpos = line.find("<FullName>").unwrap();
                     let lcurrpos1 = line.find("</FullName>").unwrap();
           		     let llen = lcurrpos1 - lcurrpos - 10;
         			 lcurrpos = lcurrpos + 10;
         		     if (lcurrpos1 != 0) & (llen > 0) {
         			     nameval = line.get(lcurrpos..(lcurrpos+llen)).unwrap().to_string();
         			     if slevel == "Dir" {
         			         sdir = nameval.to_string();
         			     }
         			 }                       
                } else {
                     if slevel == "File" {
                         if line.contains("<Date>") {
                             let mut lcurrpos = line.find("<Date>").unwrap();
                             let lcurrpos1 = line.find("</Date>").unwrap();
        				     let llen = lcurrpos1 - lcurrpos - 6;
         				     lcurrpos = lcurrpos + 6;
         				     let nameval;
         					 if (lcurrpos1 != 0) & (llen > 0) {
         					     nameval = line.get(lcurrpos..(lcurrpos+llen)).unwrap();
                                 sdate = nameval.to_string();
                             } else {
                                 sdate = "***no /Date or null value***".to_string();
                             }
                         } else if line.contains("<Size>") {
                             let mut lcurrpos = line.find("<Size>").unwrap();
                             let lcurrpos1 = line.find("</Size>").unwrap();
        					 let llen = lcurrpos1 - lcurrpos - 6;
         					 lcurrpos = lcurrpos + 6;
         					 let nameval;
         					 if (lcurrpos1 != 0) & (llen > 0) {
         					     nameval = line.get(lcurrpos..(lcurrpos+llen)).unwrap();
                                 ssize = nameval.to_string();
                             } else {
                                 ssize = "***no /Date or null value***".to_string();
                             }
                         }
                     }
                 }
                 if linenum > numrows {
                     break;
                 }
                 line.clear();
            }
            Err(_err) => {
               errstring = "error reading xml file: do file i and iconv".to_string();
               errcode = 1;
               break;
            }
         }
     }
     Ok(Execx {
            errcd: errcode,
            errval: errstring,
        })
    }
}

#[derive(Debug, Clone)]
pub enum Error {
//    APIError,
//    LanguageError,
}

// loop thru by sleeping for 5 seconds
#[derive(Debug, Clone)]
pub struct Progstart {
//    errcolor: Color,
//    errval: String,
}

impl Progstart {

    pub async fn pstart() -> Result<Progstart, Error> {
     sleep(timeDuration::from_secs(5));
     Ok(Progstart {
        })
    }
}
