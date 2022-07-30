use gdapi::model::level::{CopyType, DemonDifficulty, Difficulty, Level, Rating};

pub fn difficulty(level: &Level) -> String {
	match level.difficulty {
		Difficulty::Demon(demon) => format!("{:?} Demon", demon),
		difficulty => format!("{:?}", difficulty),
	}
}

pub fn rating(level: &Level) -> String {
	let stars = match level.stars {
		0 => "".into(),
		stars => format!(" ({} <:gdstar:831285660841148487>)", stars),
	};

	match level.rating {
		Rating::StarRated => format!("Star Rated{}", stars),
		rating => format!("{:?}{}", rating, stars),
	}
}

pub fn copy_type(level: &Level) -> String {
	match level.copy_type.unwrap() {
		CopyType::Password(password) => format!("Password ({:04})", password),
		copy_type => format!("{:?}", copy_type),
	}
}

pub fn thumbnail(level: &Level) -> String {
	let difficulty = match level.difficulty {
		Difficulty::Unrated => "unrated",
		Difficulty::Auto => "auto",
		Difficulty::Easy => "easy",
		Difficulty::Normal => "normal",
		Difficulty::Hard => "hard",
		Difficulty::Harder => "harder",
		Difficulty::Insane => "insane",
		Difficulty::Demon(demon) => match demon {
			DemonDifficulty::Easy => "demon-easy",
			DemonDifficulty::Medium => "demon-medium",
			DemonDifficulty::Hard => "demon-hard",
			DemonDifficulty::Insane => "demon-insane",
			DemonDifficulty::Extreme => "demon-extreme",
		},
	};

	let rating = match level.rating {
		Rating::None => "",
		Rating::StarRated => "",
		Rating::Featured => "-featured",
		Rating::Epic => "-epic",
	};

	format!("https://gdbrowser.com/assets/difficulties/{}{}.png", difficulty, rating)
}
