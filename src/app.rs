use crate::db::{Database, CommandModel, GroupModel};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AppScreen {
    Dashboard,
    ListCommands,
    AddCommand,
    AddGroup,
    UpdateCommandList,
    UpdateCommandForm,
    UpdateGroupForm,
    DeleteCommandList,
    ExportMenu,
    ImportForm,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FormField {
    Title,
    Description,
    Script,
    Group,
    Save,
    Cancel,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GroupFormField {
    Name,
    Description,
    CommandsList,
    AvailableCommands,
    Save,
    Cancel,
}

pub struct UsedItem {
    pub name: String,
    pub is_group: bool,
    pub use_count: u32,
}

pub struct App {
    pub screen: AppScreen,
    pub db: Database,
    pub should_quit: bool,
    
    // Navigation / lists
    pub list_selected: usize,
    pub list_tab: usize, // 0 for Single, 1 for Group
    
    // Single Command Form fields
    pub form_title: String,
    pub form_desc: String,
    pub form_script: String,
    pub form_group: String,
    pub form_focus: FormField,
    pub form_error: Option<String>,
    
    // Group Form fields
    pub group_name: String,
    pub group_desc: String,
    pub group_commands: Vec<String>,
    pub group_focus: GroupFormField,
    pub group_commands_selected: usize,
    pub group_avail_selected: usize,
    pub group_error: Option<String>,
    
    // Popup deletion confirmation
    pub delete_confirm_title: Option<String>,
    pub delete_confirm_group: bool,
    
    // Update tracking
    pub update_target_title: String,
    pub update_target_group_name: String,
    
    // Export state
    pub export_selected: usize,
    pub export_message: Option<String>,
    
    // Import state
    pub import_path: String,
    pub import_message: Option<String>,
    
    pub tick_count: u64,
}

impl App {
    pub fn new(screen: AppScreen) -> Self {
        let db = Database::load();
        Self {
            screen,
            db,
            should_quit: false,
            list_selected: 0,
            list_tab: 0,
            form_title: String::new(),
            form_desc: String::new(),
            form_script: String::new(),
            form_group: String::new(),
            form_focus: FormField::Title,
            form_error: None,
            group_name: String::new(),
            group_desc: String::new(),
            group_commands: vec![],
            group_focus: GroupFormField::Name,
            group_commands_selected: 0,
            group_avail_selected: 0,
            group_error: None,
            delete_confirm_title: None,
            delete_confirm_group: false,
            update_target_title: String::new(),
            update_target_group_name: String::new(),
            export_selected: 0,
            export_message: None,
            import_path: String::new(),
            import_message: None,
            tick_count: 0,
        }
    }

    pub fn init_form_empty(&mut self) {
        self.form_title = String::new();
        self.form_desc = String::new();
        self.form_script = String::new();
        self.form_group = String::new();
        self.form_focus = FormField::Title;
        self.form_error = None;
    }

    pub fn init_form_edit(&mut self, cmd: &CommandModel) {
        self.form_title = cmd.title.clone();
        self.form_desc = cmd.description.clone();
        self.form_script = cmd.script.clone();
        self.form_group = cmd.group.clone().unwrap_or_default();
        self.form_focus = FormField::Title;
        self.form_error = None;
        self.update_target_title = cmd.title.clone();
    }

    pub fn init_group_form_empty(&mut self) {
        self.group_name = String::new();
        self.group_desc = String::new();
        self.group_commands = vec![];
        self.group_focus = GroupFormField::Name;
        self.group_commands_selected = 0;
        self.group_avail_selected = 0;
        self.group_error = None;
    }

    pub fn init_group_form_edit(&mut self, grp: &GroupModel) {
        self.group_name = grp.name.clone();
        self.group_desc = grp.description.clone();
        self.group_commands = grp.commands.clone();
        self.group_focus = GroupFormField::Name;
        self.group_commands_selected = 0;
        self.group_avail_selected = 0;
        self.group_error = None;
        self.update_target_group_name = grp.name.clone();
    }
}
