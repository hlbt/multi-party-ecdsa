package com.example.mpc;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.os.Handler;
import android.os.HandlerThread;
import android.os.Message;
import android.util.Log;

import com.bxyz.mpc.Native;
import com.bxyz.mpc.NativeCallback;

import java.util.Date;

public class MainActivity extends AppCompatActivity {

    HandlerThread thread1;
    Handler mHandler;

    String key;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        thread1 = new HandlerThread("test1");
        thread1.start();
        Native.showLog();
        mHandler = new Handler(thread1.getLooper(), new Handler.Callback() {
            @Override
            public boolean handleMessage(@NonNull Message msg) {
                switch (msg.what) {
                    case 0: {
                        String result = Native.createKey(1, "http://api-beta.crescentbase.com:8000", "testKeygen");
                        Log.d("PPYang", "result ===========:" + result);
                        key = result;
                        break;
                    }
                    case 1: {
                        if (key != null) {
                            Log.d("PPYang", "start signData");
                            int[] parties = {1, 2};
                            String result = Native.signData("http://192.168.3.57:8000", "testKeygen", parties, "hello", key);
                            Log.d("PPYang", "start signData result:" +  result);
                        }
                        break;
                    }
                }

                return true;
            }
        });


        findViewById(R.id.create).setOnClickListener(v -> {
            mHandler.sendEmptyMessage(0);
//            test2();
        });

        findViewById(R.id.sign).setOnClickListener(v -> {
            mHandler.sendEmptyMessage(1);
        });
    }

    public void test2() {
        String room = "testKeygen1";
        new Thread("test1") {
            @Override
            public void run() {
                String result = Native.createKey(1, "http://api-beta.crescentbase.com:8000", room);
                Log.d("PPYang", "test1 result ===========:" + result);
            }
        }.start();

        new Thread("test2") {
            @Override
            public void run() {
                try {
                    Thread.sleep(100);
                } catch (InterruptedException e) {
                    throw new RuntimeException(e);
                }
                String result = Native.createKey(2, "http://api-beta.crescentbase.com:8000", room);
                Log.d("PPYang", "test2 result ===========:" + result);
            }
        }.start();

        new Thread("test3") {
            @Override
            public void run() {
                try {
                    Thread.sleep(200);
                } catch (InterruptedException e) {
                    throw new RuntimeException(e);
                }
                String result = Native.createKey(3, "http://api-beta.crescentbase.com:8000", room);
                Log.d("PPYang", "test3 result ===========:" + result);
            }
        }.start();

    }
}