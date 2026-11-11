project('dinghy', 'rust', 
          version: '0.1.0',
    meson_version: '>= 1.0.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

i18n = import('i18n')
gnome = import('gnome')

application_id = get_option('application-id')
application_conf = configuration_data()
application_conf.set('APPLICATION_ID', application_id)
application_conf.set('APPLICATION_PATH', '/' + application_id.replace('.', '/') + '/')

subdir('data')
subdir('src')
subdir('po')

gnome.post_install(
     glib_compile_schemas: true,
    gtk_update_icon_cache: true,
  update_desktop_database: true,
)
