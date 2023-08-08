//
//  ContentView.swift
//  test_mpc
//
//  Created by PPYang on 2023/8/7.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text("Hello, world!")
            Button {
                createKey()
            } label: {
                Text("createKey")
            }
        }
        .padding()
    }
    
    func createKey() {
        let address = String("http://192.168.3.57:8000");
        let room = String("testKeygen");
        let index: Int32 = 1;
        let threshold: Int32 = 1;
        let number_of_parties: Int32 = 3;
        let result = create_key(address, room, index, threshold, number_of_parties);
        print("createKey:" +  String(cString: result!))
        
        
        
        let local_share = String(cString: result!);
        startSignData(address: address, local_share:local_share);

        
    }
    
    func startSignData(address: String, local_share: String) {
        let room = String("testKeygen2");
        let signData = String("hello");
        let parties: [Int32] = [1,2];
        
        let result = sign_data(address, room, parties, signData, local_share);
        print("startSignData:" +  String(cString: result!))
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
