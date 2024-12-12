#[derive(Clone, Copy)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub enum Lang {
    EN,
}

impl Lang {
    pub fn enable(self) -> &'static str {
        match self {
            Lang::EN => en::ENABLE,
        }
    }

    pub fn radius(self) -> &'static str {
        match self {
            Lang::EN => en::RADIUS,
        }
    }
    
    pub fn color(self) -> &'static str {
        match self {
            Lang::EN => en::COLOR,
        }
    }

    pub fn config(self) -> &'static str {
        match self {
            Lang::EN => en::CONFIG,
        }
    }
    
    pub fn config_load(self) -> &'static str {
        match self {
            Lang::EN => en::CONFIG_LOAD,
        }
    }
    
    pub fn config_delete(self) -> &'static str {
        match self {
            Lang::EN => en::CONFIG_DELETE,
        }
    }
    
    pub fn config_save(self) -> &'static str {
        match self {
            Lang::EN => en::CONFIG_SAVE,
        }
    }
    
    pub fn config_create(self) -> &'static str {
        match self {
            Lang::EN => en::CONFIG_CREATE,
        }
    }
    
    pub fn config_default(self) -> &'static str {
        match self {
            Lang::EN => en::CONFIG_DEFAULT,
        }
    }
    
    pub fn repository(self) -> &'static str {
        match self {
            Lang::EN => en::REPOSITORY,
        }
    }
    
    pub fn close(self) -> &'static str {
        match self {
            Lang::EN => en::CLOSE,
        }
    }
    
    pub fn aim_not_calibrated(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_NOT_CALIBRATED,
        }
    }
    
    pub fn aim_calibrate(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_CALIBRATE,
        }
    }
    
    pub fn aim_players(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_PLAYERS,
        }
    }
    
    pub fn aim_creeps(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_CREEPS,
        }
    }
    
    pub fn aim_enable(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_ENABLE,
        }
    }
    
    pub fn aim_velocity_prediction(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_VELOCITY_PREDICTION,
        }
    }
    
    pub fn aim_rcs(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_RCS,
        }
    }
    
    pub fn aim_targeting(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_TARGETING,
        }
    }
    
    pub fn aim_fov_color(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_FOV_COLOR,
        }
    }
    
    pub fn aim_fov(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_FOV,
        }
    }
    
    pub fn aim_smooth(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_SMOOTH,
        }
    }
    
    pub fn aim_max_distance(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_MAX_DISTANCE,
        }
    }
    
    pub fn aim_meters(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_METERS,
        }
    }
    
    pub fn esp_players_rect(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_PLAYERS_RECTANGLE,
        }
    }
    
    pub fn esp_players_rect_type(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_PLAYERS_RECTANGLE_TYPE,
        }
    }
    
    pub fn esp_players_rect_stroke(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_PLAYERS_RECTANGLE_STROKE,
        }
    }
    
    pub fn esp_players_rect_fill(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_PLAYERS_RECTANGLE_FILL,
        }
    }
    
    pub fn esp_players_rect_shadow(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_PLAYERS_RECTANGLE_SHADOW,
        }
    }
    
    pub fn esp_players_rect_head(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_PLAYERS_RECTANGLE_HEAD,
        }
    }
    
    pub fn esp_players_rect_stroke_value(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_PLAYERS_RECTANGLE_STROKE_VALUE,
        }
    }
    
    pub fn esp_players_rect_shadow_value(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_PLAYERS_RECTANGLE_SHADOW_VALUE,
        }
    }
    
    pub fn esp_players_rect_shadow_blur_value(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_PLAYERS_RECTANGLE_SHADOW_BLUR_VALUE,
        }
    }
    
    pub fn esp_healthbar(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_HEALTHBAR,
        }
    }
    
    pub fn esp_healthbar_bg(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_HEALTHBAR_BACKGROUND,
        }
    }
    
    pub fn esp_healthbar_value(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_HEALTHBAR_HEALTH,
        }
    }
    
    pub fn esp_healthbar_stroke(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_HEALTHBAR_STROKE,
        }
    }
    
    pub fn align_top(self) -> &'static str {
        match self {
            Lang::EN => en::ALIGN_TOP,
        }
    }
    
    pub fn align_left(self) -> &'static str {
        match self {
            Lang::EN => en::ALIGN_TOP_LEFT,
        }
    }
    
    pub fn align_right(self) -> &'static str {
        match self {
            Lang::EN => en::ALIGN_TOP_RIGHT,
        }
    }
    
    pub fn align_bottom(self) -> &'static str {
        match self {
            Lang::EN => en::ALIGN_BOTTOM,
        }
    }
    
    pub fn esp_text_shadow(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_TEXT_CONSTAST,
        }
    }
    
    pub fn esp_text_font_size(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_TEXT_FONT_SIZE,
        }
    }
    
    pub fn esp_text(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_TEXT,
        }
    }
    
    pub fn esp_text_hero_name(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_TEXT_HERO_NAME,
        }
    }
    
    pub fn esp_text_health(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_TEXT_HEALTH,
        }
    }
    
    pub fn esp_text_distance(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_TEXT_DISTANCE,
        }
    }
    
    pub fn esp_radar(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_RADAR,
        }
    }
    
    pub fn esp_radar_radius(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_RADAR_RADIUS,
        }
    }
    
    pub fn esp_radar_scale(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_RADAR_SCALE,
        }
    }
    
    pub fn esp_radar_color_enemy(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_RADAR_COLOR_ENEMY,
        }
    }
    
    pub fn esp_radar_color_teammate(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_RADAR_COLOR_TEAMMATE,
        }
    }
    
    pub fn esp_radar_color_bg(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_RADAR_COLOR_BACKGROUND,
        }
    }
    
    pub fn esp_radar_color_stroke(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_RADAR_COLOR_STROKE,
        }
    }
    
    pub fn select_key(self) -> &'static str {
        match self {
            Lang::EN => en::SELECT_KEY,
        }
    }

    pub fn bone(self) -> &'static str {
        match self {
            Lang::EN => en::BONE,
        }
    }

    pub fn bone_head(self) -> &'static str {
        match self {
            Lang::EN => en::BONE_HEAD,
        }
    }
    pub fn bone_headnneck(self) -> &'static str {
        match self {
            Lang::EN => en::BONE_HEADNNECK,
        }
    }

    pub fn bone_neck(self) -> &'static str {
        match self {
            Lang::EN => en::BONE_NECK,
        }
    }

    pub fn bone_pelvis(self) -> &'static str {
        match self {
            Lang::EN => en::BONE_PELVIS,
        }
    }

    pub fn bone_chest(self) -> &'static str {
        match self {
            Lang::EN => en::BONE_CHEST,
        }
    }
    
    pub fn esp_radar_icon_size(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_RADAR_ICON_SIZE,
        }
    }

    pub fn config_loaded(self) -> &'static str {
        match self {
            Lang::EN => en::CONFIG_LOADED,
        }
    }
    
    pub fn config_failed(self) -> &'static str {
        match self {
            Lang::EN => en::CONFIG_FAILED,
        }
    }

    pub fn config_saved(self) -> &'static str {
        match self {
            Lang::EN => en::CONFIG_SAVED,
        }
    }

    pub fn config_deleted(self) -> &'static str {
        match self {
            Lang::EN => en::CONFIG_DELETED,
        }
    }
    
    pub fn creeps(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_PRIORITY_CREEPS,
        }
    }
    
    pub fn souls(self) -> &'static str {
        match self {
            Lang::EN => en::AIM_PRIORITY_SOULS,
        }
    }
    
    pub fn icon(self) -> &'static str {
        match self {
            Lang::EN => en::ICON,
        }
    }
    
    pub fn esp_offscreen(self) -> &'static str {
        match self {
            Lang::EN => en::ESP_OFFSCREEN,
        }
    }
    
    pub fn script_enable_all(self) -> &'static str {
        match self {
            Lang::EN => en::SCRIPT_ENABLE_ALL,
        }
    }
    
    pub fn script_disable_all(self) -> &'static str {
        match self {
            Lang::EN => en::SCRIPT_DISABLE_ALL,
        }
    }
    
    pub fn script_hero(self) -> &'static str {
        match self {
            Lang::EN => en::SCRIPT_HERO,
        }
    }
}


// Burger
pub(super) mod en
{
    pub const ENABLE: &str = "Enable";
    // pub const ENABLE: &str = "Hello. Can i get a... Big mac. No pickles. Extra... uhhhh Ketchup. Medium french fries. And also one extra large sprite.. Thanks. Are you getting a gun as a gift?";
    pub const RADIUS: &str = "Radius";
    pub const ICON: &str = "Icon";
    pub const COLOR: &str = "Color";
    pub const CONFIG: &str = "Config";
    pub const CONFIG_LOAD: &str = "Load";
    pub const CONFIG_SAVE: &str = "Save";
    pub const CONFIG_CREATE: &str = "Create";
    pub const CONFIG_DELETE: &str = "Delete";
    pub const CONFIG_DEFAULT: &str = "Load default config";
    pub const REPOSITORY: &str = "Repository (source code & updates)";
    pub const CLOSE: &str = "Close";
    pub const SELECT_KEY: &str = "Change key";

    pub const ALIGN_TOP: &str = "Top";
    pub const ALIGN_TOP_LEFT: &str = "Left";
    pub const ALIGN_TOP_RIGHT: &str = "Right";
    pub const ALIGN_BOTTOM: &str = "Bottom";
    
    pub const AIM_NOT_CALIBRATED: &str = "AIM NOT CALIBRATED";
    pub const AIM_CALIBRATE: &str = "Calibrate";
    pub const AIM_PLAYERS: &str = "Players";
    pub const AIM_CREEPS: &str = "Creeps and souls";
    pub const AIM_ENABLE: &str = "Enable aim assist";
    pub const AIM_VELOCITY_PREDICTION: &str = "Velocity prediction";
    pub const AIM_RCS: &str = "RCS";
    pub const AIM_TARGETING: &str = "Only one target";
    pub const AIM_FOV_COLOR: &str = "FOV color";
    pub const AIM_FOV: &str = "FOV";
    pub const AIM_SMOOTH: &str = "Smooth";
    pub const AIM_MAX_DISTANCE: &str = "Maximum distance";
    pub const AIM_METERS: &str = "meters";
    pub const AIM_PRIORITY_CREEPS: &str = "Creeps";
    pub const AIM_PRIORITY_SOULS: &str = "Souls";

    pub const ESP_PLAYERS_RECTANGLE: &str = "Rectangle";
    pub const ESP_PLAYERS_RECTANGLE_TYPE: &str = "Rectangle type";
    pub const ESP_PLAYERS_RECTANGLE_STROKE: &str = "Rectangle stroke";
    pub const ESP_PLAYERS_RECTANGLE_FILL: &str = "Rectangle fill";
    pub const ESP_PLAYERS_RECTANGLE_HEAD: &str = "Head";
    pub const ESP_PLAYERS_RECTANGLE_SHADOW: &str = "Rectangle shadow";
    pub const ESP_PLAYERS_RECTANGLE_STROKE_VALUE: &str = "Stroke thickness";
    pub const ESP_PLAYERS_RECTANGLE_SHADOW_VALUE: &str = "Shadow size";
    pub const ESP_PLAYERS_RECTANGLE_SHADOW_BLUR_VALUE: &str = "Shadow blur";
    
    pub const ESP_HEALTHBAR: &str = "Healthbar";
    pub const ESP_HEALTHBAR_BACKGROUND: &str = "Background color";
    pub const ESP_HEALTHBAR_HEALTH: &str = "Value color";
    pub const ESP_HEALTHBAR_STROKE: &str = "Stroke color";

    pub const ESP_TEXT_CONSTAST: &str = "Shadow";
    pub const ESP_TEXT_FONT_SIZE: &str = "Font size";
    pub const ESP_TEXT: &str = "Text";
    pub const ESP_TEXT_HERO_NAME: &str = "Hero name";
    pub const ESP_TEXT_HEALTH: &str = "Health";
    pub const ESP_TEXT_DISTANCE: &str = "Distance";

    pub const ESP_RADAR: &str = "Radar";
    pub const ESP_RADAR_RADIUS: &str = "Player dot radius";
    pub const ESP_RADAR_SCALE: &str = "Scale";
    pub const ESP_RADAR_COLOR_ENEMY: &str = "Enemy color";
    pub const ESP_RADAR_COLOR_TEAMMATE: &str = "Teammate color";
    pub const ESP_RADAR_COLOR_BACKGROUND: &str = "Bacgkround color";
    pub const ESP_RADAR_COLOR_STROKE: &str = "Stroke color";
    pub const ESP_RADAR_ICON_SIZE: &str = "Icon size";
    
    pub const CONFIG_LOADED: &str = "Config loaded";
    pub const CONFIG_FAILED: &str = "Failed to load config";
    pub const CONFIG_SAVED: &str = "Config saved";
    pub const CONFIG_DELETED: &str = "Config deleted";
    
    pub const ESP_OFFSCREEN: &str = "Offscreen";

    pub const SCRIPT_ENABLE_ALL: &str = "Enable all";
    pub const SCRIPT_DISABLE_ALL: &str = "Disable all";
    pub const SCRIPT_HERO: &str = "Hero";

    pub const BONE: &str = "Bone";
    pub const BONE_HEAD: &str = "Head";
    pub const BONE_HEADNNECK: &str = "HeadnNeck";
    pub const BONE_NECK: &str = "Neck";
    pub const BONE_CHEST: &str = "Chest";
    pub const BONE_PELVIS: &str = "Pelvis";
}

