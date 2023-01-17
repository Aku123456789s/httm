//       ___           ___           ___           ___
//      /\__\         /\  \         /\  \         /\__\
//     /:/  /         \:\  \        \:\  \       /::|  |
//    /:/__/           \:\  \        \:\  \     /:|:|  |
//   /::\  \ ___       /::\  \       /::\  \   /:/|:|__|__
//  /:/\:\  /\__\     /:/\:\__\     /:/\:\__\ /:/ |::::\__\
//  \/__\:\/:/  /    /:/  \/__/    /:/  \/__/ \/__/~~/:/  /
//       \::/  /    /:/  /        /:/  /            /:/  /
//       /:/  /     \/__/         \/__/            /:/  /
//      /:/  /                                    /:/  /
//      \/__/                                     \/__/
//
// (c) Robert Swinford <robert.swinford<...at...>gmail.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.

use std::collections::BTreeMap;

use crate::config::generate::{Config};
use crate::data::paths::{PathData};
use crate::library::results::{HttmError, HttmResult};
use crate::lookup::versions::VersionsMap;
use crate::lookup::file_mounts::MountsForFiles;
use crate::parse::aliases::FilesystemType;

pub struct InteractiveWipe;

impl InteractiveWipe {
    pub fn exec(
        config: &Config,
        paths_selected_in_browse: &[PathData],
    ) -> HttmResult<()> {
        let (existing_paths, dne_paths): (Vec<PathData>, Vec<PathData>) = paths_selected_in_browse
            .iter()
            .cloned()
            .partition(|pathdata| pathdata.path_buf.exists());

        if !existing_paths.is_empty() {
            return Err(
                HttmError::new("httm does not support wiping non-deleted files. Some of your paths are non-deleted, live files. Quitting.").into(),
            );
        }

        let (non_zfs_mounts, zfs_mounts): (BTreeMap<PathData, Vec<PathData>>, BTreeMap<PathData, Vec<PathData>>)  = MountsForFiles::new(config)
            .into_iter()
            .flat_map(|(_pathdata, datasets)| datasets)
            .partition(|mount| {
                match config.dataset_collection.map_of_datasets.datasets.get(&mount.path_buf) {
                        Some(dataset_info) => {
                            if let FilesystemType::Zfs = dataset_info.fs_type {
                                false
                            } else {
                                eprintln!("Error: {:?} is an non-ZFS dataset", mount);
                                true
                            }
                        }
                        None => false,
                    }
            });
        
        if !non_zfs_mounts.is_empty() {
            return Err(
                HttmError::new("httm does not support wiping non-ZFS datasets at this time. Quitting.").into(),
            );
        }

        let version_map = VersionsMap::exec(config, &dne_paths);

        let snapshot_names_to_wipe: BTreeMap<PathData, Vec<String>>  = version_map
            .into_iter()
            .map(|(deleted_file, snap_path)| {
                let res: Vec<String> = snap_path
                    .iter()
                    .filter_map(|path| {
                       path.path_buf
                        .to_string_lossy()
                        .split_once(".zfs/snapshot/")
                        .map(|(_first, rest)| rest)
                        .map(|str| str.split_once("/"))
                        .map(||)
                    })
                    .map(|str| str.to_owned())
                    .collect();
                (deleted_file, res)
            })
            .collect();


        let requested_file_wipes = version_map.keys().collect();

        // tell the user what we're up to, and get consent
        let preview_buffer = format!(
            "User has requested httm wipe to following deleted files from all snapshots: {:?}\n\n
            Thus, httm has identified the following as snapshots as snapshot upon which the deleted files reside: {:?}\n\n\
            Before httm destroys these snapshots, it would like your consent. Continue? (YES/NO)\n\
            ──────────────────────────────────────────────────────────────────────────────\n\
            YES\n\
            NO",
            requested_file_wipes,
            snapshots_to_wipe,
        );




        Ok(())
    }
}