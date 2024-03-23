pub struct Lab {
    image_path: String,
    expanded_path: Option<String>,
    drive_letter: Option<String>,
}

pub struct LabBuilder {
    path: Option<String>,
}

impl LabBuilder {
    #[inline(always)]
    pub fn path(mut self, path: String) -> Self {
        self.path = Some(path);

        self
    }
}

impl Lab {
    #[inline(always)]
    pub fn builder() -> LabBuilder {
        LabBuilder { path: None }
    }

    #[inline(always)]
    pub fn from_image(path: String) -> Self {
        Self {
            image_path: path,
            expanded_path: None,
            drive_letter: None,
        }
    }

    pub fn expand(&mut self, path: String) -> Result<(), &'static str> {
        Err("Not implemented!")
    }

    pub fn mount(&mut self, drive_letter: String) -> Result<(), &'static str> {
        if let Some(expanded_path) = &self.expanded_path {
            match Self::create_volume(&drive_letter, &expanded_path) {
                true => {
                    self.drive_letter = Some(drive_letter);

                    return Ok(())
                }
                false => return Err("Failed to create volume!"),
            }
        }

        Err("Lab is not expanded!")
    }

    pub fn unmount(&mut self) -> Result<(), &'static str> {
        if let Some(drive_letter) = &self.drive_letter {
            match Self::delete_volume(&drive_letter) {
                true => {
                    self.drive_letter = None;

                    return Ok(());
                }
                false => return Err("Failed to delete volume!"),
            }
        }

        Err("Lab is not mounted!")
    }

    #[inline(always)]
    fn create_volume(drive_letter: &str, path: &str) -> bool {
        win_subst::add(drive_letter, path)
    }

    #[inline(always)]
    fn delete_volume(drive_letter: &str) -> bool {
        win_subst::del(drive_letter)
    }
}
