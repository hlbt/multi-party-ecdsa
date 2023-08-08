//
//  Mpc.swift
//  test_mpc
//
//  Created by PPYang on 2023/8/7.
//

import Foundation


class Mpc {
    func createKey() {
        let result = create_key("http://192.168.3.57:8000", "testKeygen", 1, 1, 3);
        print("createKey:" +  result)
    }
}

