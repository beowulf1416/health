// This file can be replaced during build by using the `fileReplacements` array.
// `ng build` replaces `environment.ts` with `environment.prod.ts`.
// The list of file replacements can be found in `angular.json`.

export const environment = {
  production: false,

  key_session_token: 'token',
  url_base: 'http://localhost:8081',

  path_authenticate: '/user/authenticate',
  path_sign_out: '/user/signout',

  path_domain_add: '/domain/add',
  path_domain_get: '/domain/get',
  path_domain_fetch: '/domain/fetch',
  path_domain_update: '/domain/update',
  path_domain_set_active: '/domain/set/active',

  path_user_add: '/user/add',
  path_user_fetch: '/user/fetch',
  path_user_get: '/user/get',
  path_user_set_active: '/user/set/active',
  path_user_set_password: '/user/set/password',
  path_user_update: '/user/update',
  path_user_current: '/user/current',

  path_role_add: '/role/add',
  path_role_get: '/role/get',
  path_role_fetch: '/role/fetch',
  path_role_update: '/role/update',
  path_role_set_active: '/role/set/active'
};

/*
 * For easier debugging in development mode, you can import the following file
 * to ignore zone related error stack frames such as `zone.run`, `zoneDelegate.invokeTask`.
 *
 * This import should be commented out in production mode because it will have a negative impact
 * on performance if an error is thrown.
 */
// import 'zone.js/plugins/zone-error';  // Included with Angular CLI.
