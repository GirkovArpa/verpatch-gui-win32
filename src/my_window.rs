#![macro_use]
use winsafe::gui;
use winsafe::WinResult;

#[derive(Clone)]
pub struct MyWindow {
    wnd: gui::WindowMain,
    edits: Vec<gui::Edit>,
    labels: Vec<gui::Label>,
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
            "Original Filename",
        ];

        let edit_values: Vec<&str> = vec!["", "", "0.0.0.0", "", "0.0.0.0", "", "0.0.0.0", "", ""];

        let mut edits: Vec<gui::Edit> = vec![];
        let mut labels: Vec<gui::Label> = vec![];

        for i in 0..9 {
            let edit = gui::Edit::new(
                &wnd,
                gui::EditOpts {
                    text: format!("{}", edit_values[i as usize]),
                    position: winsafe::POINT::new(120, 5 + (20 * i) + (5 * i)),
                    width: 175,
                    ..Default::default()
                },
            );

            let label = gui::Label::new(
                &wnd,
                gui::LabelOpts {
                    text: format!("{}:", label_texts[i as usize]),
                    position: winsafe::POINT::new(5, 5 + (20 * i) + (5 * i)),
                    size: winsafe::SIZE::new(105, 20),
                    label_style: winsafe::co::SS::RIGHT | winsafe::co::SS::NOTIFY,
                    ..Default::default()
                },
            );

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
        self.wnd.on().wm_command(winsafe::co::CMD::Menu, 1, {
            let myself = self.clone();
            move || {
                println!("Open clicked.");
                let fileo: winsafe::shell::IFileOpenDialog = winsafe::CoCreateInstance(
                    &winsafe::shell::clsid::FileOpenDialog,
                    None,
                    winsafe::co::CLSCTX::INPROC_SERVER,
                )
                .unwrap();

                fileo
                    .IFileDialog
                    .SetOptions(
                        fileo.IFileDialog.GetOptions().unwrap()
                            | winsafe::co::FOS::FORCEFILESYSTEM
                            | winsafe::co::FOS::FILEMUSTEXIST,
                    )
                    .unwrap();

                fileo
                    .IFileDialog
                    .SetFileTypes(&[("Binary files", "*.exe;*.dll"), ("All files", "*.*")])
                    .unwrap();

                fileo.IFileDialog.SetFileTypeIndex(0).unwrap();

                let user_clicked_ok = fileo
                    .IFileDialog
                    .IModalWindow
                    .Show(myself.wnd.hwnd())
                    .unwrap();

                if user_clicked_ok {
                    let shi = fileo.IFileDialog.GetResult().unwrap();
                    let file_path = shi.GetDisplayName(winsafe::co::SIGDN::FILESYSPATH).unwrap();
                    myself.edits[0].set_text(&file_path).unwrap();
                }
            }
        });

        self.wnd.on().wm_command(winsafe::co::CMD::Menu, 2, {
            let myself = self.clone();
            move || {
                println!("Save clicked.");
                let edits = &myself.edits;

                let filename = &edits[0].text().unwrap();
                let file_description = &edits[1].text().unwrap();
                let file_version = &edits[2].text().unwrap();
                let product_name = &edits[3].text().unwrap();
                let product_version = &edits[4].text().unwrap();
                let legal_copyright = &edits[5].text().unwrap();
                let private_build = &edits[6].text().unwrap();
                let company_name = &edits[7].text().unwrap();
                let original_filename = &edits[8].text().unwrap();

                use std::os::windows::process::CommandExt;
                use std::process::Command;
                const CREATE_NO_WINDOW: u32 = 0x08000000;
                let output = Command::new("verpatch")
                    .creation_flags(CREATE_NO_WINDOW)
                    .args(&[
                        "/va",
                        filename,
                        file_version,
                        "/s",
                        "OriginalFilename",
                        original_filename,
                        "/s",
                        "desc",
                        file_description,
                        "/s",
                        "pb",
                        private_build,
                        "/s",
                        "company",
                        company_name,
                        "/s",
                        "(c)",
                        legal_copyright,
                        "/s",
                        "product",
                        product_name,
                        "/pv",
                        product_version,
                    ])
                    .output();

                match output {
                    Ok(output) => {
                        if output.status.success() {
                            myself
                                .wnd
                                .hwnd()
                                .MessageBox(
                                    "File metadata updated successfully!",
                                    "Saved!",
                                    winsafe::co::MB::OK,
                                )
                                .unwrap();
                            return;
                        }
                        myself
                            .wnd
                            .hwnd()
                            .MessageBox(
                                &format!(
                                    "Save failed:\n{}",
                                    std::str::from_utf8(&output.stdout).unwrap()
                                ),
                                "Error",
                                winsafe::co::MB::OK | winsafe::co::MB::ICONERROR,
                            )
                            .unwrap();
                    }
                    Err(e) => {
                        myself
                            .wnd
                            .hwnd()
                            .MessageBox(
                                &format!("Unable to execute verpatch.exe:\n{}", e.to_string()),
                                "Error",
                                winsafe::co::MB::OK | winsafe::co::MB::ICONERROR,
                            )
                            .unwrap();
                    }
                }
            }
        });

        self.wnd.on().wm_command(winsafe::co::CMD::Menu, 3, {
            let myself = self.clone();
            move || {
                println!("Usage clicked.");
                myself
                    .wnd
                    .hwnd()
                    .MessageBox(
                        "Verpatch.exe must be in this folder,\nor in your PATH environment variable.",
                        "Usage",
                        winsafe::co::MB::OK | winsafe::co::MB::ICONINFORMATION,
                    )
                    .unwrap();
            }
        });

        self.wnd.on().wm_command(winsafe::co::CMD::Menu, 4, {
            let myself = self.clone();
            move || {
                println!("About clicked.");
                myself
                    .wnd
                    .hwnd()
                    .MessageBox(
                        "Verpatch GUI Win32 v0.1.0\n© 2021 GirkovArpa\n\nMade w/ winsafe crate",
                        "About",
                        winsafe::co::MB::OK | winsafe::co::MB::ICONINFORMATION,
                    )
                    .unwrap();
            }
        });
    }
}
