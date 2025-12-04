{ lib, ... }:
{
  diskoConfig = lib.recursiveUpdate (import ../disko.nix) {
    disko.devices.zpool."zroot".datasets."ds1/persist".options.":test" = "letsgo";
  };

  diskoZfs = {
    ignoredDatasets = [ "zroot/ds1/persist/postgresql" ];

    datasets = {
      "zroot/ds1/persist/postgresql" = { };
    };
  };

  extraTestScript = ''
    assert_zfs_dataset_not_exists("zroot/ds1/persist/postgresql")
  '';
}
