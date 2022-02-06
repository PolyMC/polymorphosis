# polymorphosis

Meta file generator for PolyMC.

## Why is this needed? Aren't the original meta scripts working just fine?

Well yes and no. While it's true that they work, they're badly written and almost unmaintanable due to a lack of proper documentation.

I'm making the effort to fully explain what is going on in the scripts and rewrite them as a Rust application.
Rewriting the metadata generation in Rust is faster and easily deployable.

Another reason is that PolyMC might use a different meta format in the future. In which case this repository will be updated.

## How do the old meta scripts work?

1. Clone [upstream](https://github.com/PolyMC/meta-upstream) and [meta](https://github.com/PolyMC/meta-polymc) repositories, if they don't exist already.
2. Hard-reset the local upstream repo and check out the configured branch (usually either master or develop).
3. Update Mojang metadata.
    1. Try to load the local version list in the upstream repo under `mojang/version_manifest_v2.json`, if it exists.
    2. Download Mojang's current version list from `https://launchermeta.mojang.com/mc/game/version_manifest_v2.json`.
    3. Check which versions are not present locally.
    4. Check which versions are present both locally and remotely.
    5. Compare the times of the locally and remotely present versions. If the time of the remote version is newer, flag the version to be updated in the local version list.
    6. Download the version file for each version that is new or needs to be updated and save them in the upstream repo under `mojang/versions/{version_id}.json`.
    7. Download the asset meta files for each version and save them in the upstream repo under `mojang/assets/{asset_id}.json`.
    8. Save the remote version list in the upstream repo under `mojang/version_manifest_v2.json`
4. Update Forge metadata.
    1. Download Forge's Maven metadata from `https://files.minecraftforge.net/net/minecraftforge/forge/maven-metadata.json`.
    2. Download Forge's slim promotions list from `https://files.minecraftforge.net/net/minecraftforge/forge/promotions_slim.json`.
    3. Collect all recommended versions via the promotions in a list.
    4. For each MC version in the Maven metadata, make sure that each MC version has an array of Forge versions assigned.
    5. Go through each Forge version listed, check if the Forge version is valid (Regex match for `^(?P<mc>[0-9a-zA-Z_\\.]+)-(?P<ver>[0-9\\.]+\\.(?P<build>[0-9]+))(-(?P<branch>[a-zA-Z0-9\\.]+))?$`) and download the manifest file (since this step also is a little bigger, I will subsection here again):
        1. Check if the manifest file exists on disk in the upstream repo under `forge/files_manifests/{long_version}.json`. If it doesn't, download it from `https://files.minecraftforge.net/net/minecraftforge/forge/{long_version}/meta.json` (`long_version` being a full Forge version string like `1.18.1-39.0.64`).
        2. Go through each classifier of the meta file and replace anything that isn't a valid hash character.
        3. If the hash isn't 32 characters long, the hash is considered invalid and skipped.
        4. If the manifest file didn't exist on the disk, it is now saved under the name from step 1.
        5. Return a list of file info containing the classifier, hash and extension.
    6. Generate a derived index, containing the metadata for each version, which Forge version is recommended, which version is the latest version and a list of Forge versions that belongs to each MC versions.
    7. Save the downloaded Maven metadata (`forge/maven-metadata.json`), promotion data (`forge/promotion_slim.json`) and the derived index (`forge/derived_index.json`) to the respective file locations.
    8. Download each Forge installer, if it doesn't exist on disk already.
    9. Access each installer, save the `version.json` inside under `forge/version_manifests/{long_version}.json`, try to parse the `installer_profile.json` and save it under `forge/installer_manifests/{long_version}.json` and generate a SHA1 and SHA256 hash digest and save those with the installer file size to `forge/installer_info/{long_version}.json`.
    10. Check if `static/forge-legacyinfo.json` exists and if it doesn't, download the legacy versions and generate legacy info by finding the newest file inside the JAR file, getting the timestamp of the file and generate SHA1 and SHA256 digests and save them under `static/forge-legacyinfo.json`.
5. Update Fabric metadata.
6. Update Liteloader metadata.
7. If any of the steps 3-6 failed, hard-reset the upstream repo and exit with exit code 1.
8. Hard-reset the meta repository and check out the configured branch (usually either master or develop)
9. Generate PolyMC metadata from Mojang metadata.
10. Generate PolyMC metadata from Forge metadata.
11. Generate PolyMC metadata from Fabric metadata.
12. Generate PolyMC metadata from Liteloader metadata.
13. Generate a PolyMC index file.
