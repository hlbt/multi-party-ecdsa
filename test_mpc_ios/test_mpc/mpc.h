//
//  mpc.h
//  test_mpc
//
//  Created by PPYang on 2023/8/7.
//

#ifndef mpc_h
#define mpc_h
#include <stdint.h>

const char* create_key(const char* address, const char* room, const int index, const int threshold, const int number_of_parties);


const char* sign_data(const char* address, const char* room, const int* parties, const char* data_to_sign, const char* local_share);

const void rust_free(const char* s);


#endif /* mpc_h */
