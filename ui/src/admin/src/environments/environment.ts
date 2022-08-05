// This file can be replaced during build by using the `fileReplacements` array.
// `ng build` replaces `environment.ts` with `environment.prod.ts`.
// The list of file replacements can be found in `angular.json`.

export const environment = {
  production: false,

  key_session_token: 'token',
  url_base: 'http://localhost:8081',
  path_authenticate: '/user/authenticate',

  path_domain_add: '/domain/add',
  path_domain_list: '/domain/list',

  path_user_add: '/user/add',
  path_user_list: '/user/list',

  path_role_add: '/role/add',
  path_role_list: '/role/list'
};

/*
 * For easier debugging in development mode, you can import the following file
 * to ignore zone related error stack frames such as `zone.run`, `zoneDelegate.invokeTask`.
 *
 * This import should be commented out in production mode because it will have a negative impact
 * on performance if an error is thrown.
 */
// import 'zone.js/plugins/zone-error';  // Included with Angular CLI.
