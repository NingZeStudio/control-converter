package com.tungsten.fcl.jnitest;

import android.app.Activity;
import android.os.Bundle;
import android.os.Environment;
import android.text.method.ScrollingMovementMethod;
import android.view.Gravity;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.LinearLayout;
import android.widget.ScrollView;
import android.widget.TextView;

import com.tungsten.fcl.util.LayoutConverter;

import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;

/**
 * Minimal JNI smoke test for the Go conversion library (libcc.so).
 *
 * Layout:
 *   [input path]  (default: bundled test_fcl_layout.json copied to app files dir)
 *   [output path] (default: files dir / output_zl2.json)
 *   [Convert] button
 *   [result] scrollable text view
 */
public class MainActivity extends Activity {

    private static final String ASSET_INPUT = "test_fcl_layout.json";

    private EditText inputPathEdit;
    private EditText outputPathEdit;
    private TextView resultText;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(buildUi());

        File input = new File(getFilesDir(), ASSET_INPUT);
        if (!input.exists()) {
            copyAssetToFile(ASSET_INPUT, input);
        }
        File output = new File(getFilesDir(), "output_zl2.json");

        inputPathEdit.setText(input.getAbsolutePath());
        outputPathEdit.setText(output.getAbsolutePath());
    }

    private View buildUi() {
        LinearLayout root = new LinearLayout(this);
        root.setOrientation(LinearLayout.VERTICAL);
        int pad = dp(12);
        root.setPadding(pad, pad, pad, pad);

        root.addView(label("FCL layout JSON 输入路径"));
        inputPathEdit = new EditText(this);
        inputPathEdit.setSingleLine(true);
        root.addView(inputPathEdit);

        root.addView(label("ZL2 输出路径"));
        outputPathEdit = new EditText(this);
        outputPathEdit.setSingleLine(true);
        root.addView(outputPathEdit);

        Button convert = new Button(this);
        convert.setText("转换 (convertFclToZl2Native)");
        convert.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                runConversion();
            }
        });
        root.addView(convert);

        resultText = new TextView(this);
        resultText.setMovementMethod(new ScrollingMovementMethod());
        resultText.setTextSize(11);
        resultText.setTextColor(0xFF222222);
        resultText.setText("就绪。点击转换按钮调用 JNI。");

        ScrollView scroll = new ScrollView(this);
        scroll.addView(resultText);
        LinearLayout.LayoutParams scrollLp = new LinearLayout.LayoutParams(
                LinearLayout.LayoutParams.MATCH_PARENT, 0, 1f);
        root.addView(scroll, scrollLp);
        return root;
    }

    private TextView label(String text) {
        TextView tv = new TextView(this);
        tv.setText(text);
        tv.setTextSize(13);
        tv.setTextColor(0xFF555555);
        tv.setPadding(0, dp(8), 0, dp(2));
        return tv;
    }

    private int dp(int value) {
        return Math.round(getResources().getDisplayMetrics().density * value);
    }

    private void runConversion() {
        final String input = inputPathEdit.getText().toString().trim();
        final String output = outputPathEdit.getText().toString().trim();

        resultText.setText("转换中…\n输入: " + input + "\n输出: " + output);

        // Native conversion blocks; run off the UI thread.
        new Thread(new Runnable() {
            @Override
            public void run() {
                final StringBuilder report = new StringBuilder();
                try {
                    long start = System.currentTimeMillis();
                    LayoutConverter.convertFclToZl2(input, output);
                    long elapsed = System.currentTimeMillis() - start;

                    File out = new File(output);
                    report.append("✅ 转换成功 (").append(elapsed).append(" ms)\n");
                    report.append("输出文件: ").append(out.getAbsolutePath()).append("\n");
                    report.append("大小: ").append(out.length()).append(" bytes\n\n");
                    report.append("--- 输出预览 ---\n").append(preview(out, 4000));
                } catch (Throwable t) {
                    report.append("❌ 转换失败\n").append(t.toString());
                }
                runOnUiThread(new Runnable() {
                    @Override
                    public void run() {
                        resultText.setText(report.toString());
                    }
                });
            }
        }).start();
    }

    private String preview(File file, int maxChars) {
        // Avoid java.nio.file.Files (API 26+); use plain streams for minSdk 21.
        try (InputStream in = new java.io.FileInputStream(file);
             java.io.ByteArrayOutputStream bos = new java.io.ByteArrayOutputStream()) {
            byte[] buf = new byte[8192];
            int len;
            while ((len = in.read(buf)) > 0) {
                bos.write(buf, 0, len);
            }
            byte[] bytes = bos.toByteArray();
            int n = Math.min(bytes.length, maxChars);
            return new String(bytes, 0, n, "UTF-8")
                    + (bytes.length > n ? "\n… (截断, 共 " + bytes.length + " 字节)" : "");
        } catch (IOException e) {
            return "(无法读取输出: " + e + ")";
        }
    }

    private void copyAssetToFile(String asset, File target) {
        try (InputStream in = getAssets().open(asset);
             FileOutputStream out = new FileOutputStream(target)) {
            byte[] buf = new byte[8192];
            int len;
            while ((len = in.read(buf)) > 0) {
                out.write(buf, 0, len);
            }
        } catch (IOException e) {
            resultText.setText("复制 asset 失败: " + e);
        }
    }
}
