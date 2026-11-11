scalable_dir = 'hicolor' / 'scalable' / 'apps'
install_data(
  scalable_dir / ('@0@.svg').format(application_id),
  install_dir: get_option('datadir') / 'icons' / scalable_dir
)

symbolic_dir = 'hicolor' / 'symbolic' / 'apps'
install_data(
  symbolic_dir / 'io.github.jpadgett314.Dinghy-symbolic.svg',
  rename: application_id + '-symbolic.svg',
  install_dir: get_option('datadir') / 'icons' / symbolic_dir
)
