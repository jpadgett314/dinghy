desktop_input = configure_file(
  input: 'io.github.jpadgett314.Dinghy.desktop.in',
  output: application_id + '.desktop.in',
  configuration: application_conf,
)

desktop_file = i18n.merge_file(
        input: desktop_input,
       output: application_id + '.desktop',
         type: 'desktop',
       po_dir: '../po',
      install: true,
  install_dir: get_option('datadir') / 'applications'
)

desktop_utils = find_program('desktop-file-validate', required: false)
if desktop_utils.found()
  test('Validate desktop file', desktop_utils, args: [desktop_file])
endif

appstream_input = configure_file(
  input: 'io.github.jpadgett314.Dinghy.metainfo.xml.in',
  output: application_id + '.metainfo.xml.in',
  configuration: application_conf,
)

appstream_file = i18n.merge_file(
        input: appstream_input,
       output: application_id + '.metainfo.xml',
       po_dir: '../po',
      install: true,
  install_dir: get_option('datadir') / 'metainfo'
)

appstreamcli = find_program('appstreamcli', required: false, disabler: true)
test('Validate appstream file', appstreamcli,
     args: ['validate', '--no-net', '--explain', appstream_file])

configure_file(
  input: 'io.github.jpadgett314.Dinghy.gschema.xml.in',
  output: application_id + '.gschema.xml',
  configuration: application_conf,
  install_dir: get_option('datadir') / 'glib-2.0' / 'schemas'
)

compile_schemas = find_program('glib-compile-schemas', required: false, disabler: true)
test('Validate schema file',
     compile_schemas,
     args: ['--strict', '--dry-run', meson.current_build_dir()])

service_conf = configuration_data()
service_conf.set('APPLICATION_ID', application_id)
service_conf.set('bindir', get_option('prefix') / get_option('bindir'))
configure_file(
  input: 'io.github.jpadgett314.Dinghy.service.in',
  output: application_id + '.service',
  configuration: service_conf,
  install_dir: get_option('datadir') / 'dbus-1' / 'services'
)

subdir('icons')
