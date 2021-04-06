use winsafe::gui;
use winsafe::WinResult;

#[derive(Clone)]
pub struct MyWindow {
  wnd: gui::WindowMain,
  edits: Vec<gui::Edit>,
  labels: Vec<gui::Label>
}

impl MyWindow {
  pub fn new() -> MyWindow {
    let wnd = gui::WindowMain::new_dlg(1000, Some(101), None);

    let label_texts: Vec<&str> = vec![
      "Filename",
      "File Description",
      "File Version",
      "Product Name",
      "Product Version",
      "Legal Copyright",
      "Private Build",
      "Company Name",
      "Original Filename"
    ];

    let edit_values: Vec<&str> = vec![
      "", 
      "", 
      "0.0.0.0", 
      "", 
      "0.0.0.0", 
      "", 
      "0.0.0.0", 
      "", 
      "", 
    ];

    let mut edits: Vec<gui::Edit> = vec![];
    let mut labels: Vec<gui::Label> = vec![];

    for i in 0..9 {
      let edit = gui::Edit::new(&wnd, gui::EditOpts {
        text: format!("{}", edit_values[i as usize]),
        position: winsafe::POINT::new( 150, 5 + (20 * i) + (5 * i)),
        width: 145,
        ..Default::default()
      });

      let label = gui::Label::new(&wnd, gui::LabelOpts {
        text: format!("{}:", label_texts[i as usize]),
        position: winsafe::POINT::new( 5, 5 + (20 * i) + (5 * i)),
        // the line below requires winsafe 0.0.3 
        // size: winsafe::SIZE::new(145, 20),
        label_style: winsafe::co::SS::RIGHT | winsafe::co::SS::NOTIFY,
        ..Default::default()
      });

      edits.push(edit);
      labels.push(label);
    } 
    
    let new_self = Self { wnd, edits, labels };

    new_self.events();
    new_self
  }

  pub fn run(&self) -> WinResult<()> {
    self.wnd.run_main(None)
  }

  fn events(&self) {
    self.wnd.on().wm_init_dialog({
      let myself = self.clone();
      move |_| {
        
        true  
      }
    });

    self.wnd.on().wm_init_menu_popup({
      let myself = self.clone();
      move |_| {
        println!("a menu was opened");
      }
    });
  }
}
