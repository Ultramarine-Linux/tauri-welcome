use serde::{Deserialize, Serialize};
use std::vec;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Copr {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub description: String,
    pub conflicts: Option<Vec<String>>,
    pub copr: Option<String>,
    pub pre_script: Option<Vec<String>>,
    pub flatpak: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub description: String,
    pub packages: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Preset {
    pub id: String,
    pub name: String,
    pub description: String,
    pub packages: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SoftwareSet {
    pub packages: Vec<Package>,
    pub extra_copr_repos: Vec<Copr>,
    pub groups: Vec<Group>,
    pub presets: Vec<Preset>,
}
/// Outputs the full default list of packages, this should not actually be used for installing, as selection is done by creating
/// a new SoftwareSet with only the packages, groups, and presets that you need.
pub fn list() -> SoftwareSet {
    let yaml_file = include_str!("software_list.yml");
    // read yaml file
    let software_set: SoftwareSet = serde_yaml::from_str(yaml_file).unwrap();
    println!("{:#?}", software_set);

    software_set.groups.iter().for_each(|group| {
        group.packages.clone().into_iter().for_each(|package| {
            // check if package with id exists
            let pkg = software_set.packages.iter().find(|p| p.id == package);
            if pkg.is_none() {
                panic!("Expected a package with id {} in group {}", package, group.id);
            }
        });
    });

    software_set.presets.clone().into_iter().for_each(|preset| {
        if preset.packages.is_none() && preset.groups.is_none() {
            panic!("Expected either packages or groups in preset {}", preset.id);
        }

        preset.packages.unwrap_or_default().into_iter().for_each(|package| {
            // check if package with id exists
            let pkg = software_set.packages.iter().find(|p| p.id == package);
            if pkg.is_none() {
                panic!("Expected a package with id {} in preset {}", package, preset.id);
            }
        });

        preset.groups.unwrap_or_default().into_iter().for_each(|group| {
            // check if group with id exists
            let grp = software_set.groups.iter().find(|g| g.id == group);
            if grp.is_none() {
                panic!("Expected a group with id {} in preset {}", group, preset.id);
            }
        });
    });

    software_set.extra_copr_repos.clone().into_iter().for_each(|repo| {
        if repo.name.is_empty() {
            panic!("Expected a name for copr repo {}", repo.description);
        }
    });

    software_set.packages.clone().into_iter().for_each(|package| {
        if let Some(copr) = package.copr {
            // check if copr repo with name exists
            let repo = software_set.extra_copr_repos.iter().find(|r| r.name == copr);
            if repo.is_none() {
                panic!("Expected a copr repo with name {} for package {}", copr, package.id);
            }
        }
    });

    software_set

}

pub fn install(set: SoftwareSet) {
    println!("Installing software set {:#?}", set);
    let mut packages_dnf = vec![];
    let mut packages_flatpak = vec![];

    for package in &set.packages {
        for pre_script in package.pre_script.clone().unwrap_or_default() {
            println!("$ {}", pre_script);
        }
        if package.flatpak.unwrap_or_default() {
            packages_flatpak.push(package.id.clone());
        } else {
            packages_dnf.push(package.id.clone());
        }

        if let Some(copr) = &package.copr {
            println!("$ pkexec dnf copr enable -y {} fedora-36-x86_64", copr);
        }
    }

    // check conflicts
    for package in &set.packages {
        if let Some(conflicts) = &package.conflicts {
            for conflict in conflicts {
                let pkg = set.packages.iter().find(|p| p.id == *conflict);
                if pkg.is_some() {
                    println!("Package {} conflicts with {}", package.id, conflict);
                }
            }
        }
    }
    if !packages_dnf.is_empty() {
        println!("$ pkexec dnf install -y {}", packages_dnf.join(" "));
    }

    if !packages_flatpak.is_empty() {
        println!("$ flatpak install -y {}", packages_flatpak.join(" "));
    }

}


pub fn export_selection(set: SoftwareSet) {
    // serialize back to yaml
    let yaml = serde_yaml::to_string(&set).unwrap();
    println!("{}", yaml);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        install(list());
        //export_selection(list());
    }
}