//
//  test_mpcApp.swift
//  test_mpc
//
//  Created by PPYang on 2023/8/7.
//

import SwiftUI


@main
struct test_mpcApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
//        .commands {
//            CommandGroup(after: .newItem) {
//                Button {
//                    createKey()
//                } label: {
//                    Text("createKey")
//                }
//            }
//        }
    }
    
//    func createKey() {
//        let address = String("http://192.168.3.57:8000");
//        let room = String("testKeygen");
//        let index: Int32 = 1;
//        let threshold: Int32 = 1;
//        let number_of_parties: Int32 = 3;
//        let result = create_key(address, room, index, threshold, number_of_parties);
//        print("createKey:" +  String(cString: result!))
//    }
}
