/*
 Copyright 2015-2017 Intecture Developers. See the COPYRIGHT file at the
 top-level directory of this distribution and at
 https://intecture.io/COPYRIGHT.

 Licensed under the Mozilla Public License 2.0 <LICENSE or
 https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
 modified, or distributed except according to those terms.
*/

#include "php_inapi.h"
#include "src/command.h"
#include "src/directory.h"
#include "src/file.h"
#include "src/host.h"
#include "src/package.h"
#include "src/payload.h"
#include "src/service.h"
#include "src/template.h"

PHP_MINIT_FUNCTION(inapi)
{
    inapi_init_host(TSRMLS_C);
    inapi_init_host_exception(TSRMLS_C);
    inapi_init_command(TSRMLS_C);
    inapi_init_command_exception(TSRMLS_C);
    inapi_init_directory(TSRMLS_C);
    inapi_init_directory_exception(TSRMLS_C);
    inapi_init_file(TSRMLS_C);
    inapi_init_file_exception(TSRMLS_C);
    inapi_init_package(TSRMLS_C);
    inapi_init_package_exception(TSRMLS_C);
    inapi_init_payload(TSRMLS_C);
    inapi_init_payload_exception(TSRMLS_C);
    inapi_init_service(TSRMLS_C);
    inapi_init_service_exception(TSRMLS_C);
    inapi_init_service_runnable(TSRMLS_C);
    inapi_init_template(TSRMLS_C);
    inapi_init_template_exception(TSRMLS_C);
    return SUCCESS;
}

zend_module_entry inapi_module_entry = {
#if ZEND_MODULE_API_NO >= 20010901
    STANDARD_MODULE_HEADER,
#endif
    PHP_INAPI_EXTNAME,     /* Extension name */
    NULL,                  /* Functions */
    PHP_MINIT(inapi),      /* Methods */
    NULL,                  /* MSHUTDOWN */
    NULL,                  /* RINIT */
    NULL,                  /* RSHUTDOWN */
    NULL,                  /* MINFO */
#if ZEND_MODULE_API_NO >= 20010901
    PHP_INAPI_EXTVER,      /* Extension version */
#endif
    STANDARD_MODULE_PROPERTIES
};

#ifdef COMPILE_DL_INAPI
ZEND_GET_MODULE(inapi)
#endif
