package com.example.sudoku;

import androidx.annotation.NonNull;
import androidx.appcompat.app.AppCompatActivity;
import androidx.preference.PreferenceManager;

import android.content.Intent;
import android.content.SharedPreferences;
import android.content.res.Configuration;
import android.content.res.Resources;
import android.os.Bundle;
import android.view.Menu;
import android.view.MenuInflater;
import android.view.MenuItem;
import android.view.View;
import android.widget.TextView;

import java.util.Locale;

public class MainActivity extends AppCompatActivity {
    public static final String GAME_DIFFICULTY = "DIFFICULTY";
    @Override
    protected void onCreate(Bundle savedInstanceState) {

        SharedPreferences spref = PreferenceManager.getDefaultSharedPreferences(this);
        String languagePreference = spref.getString(SettingsActivity.KEY_LANGUAGE_PREF, "english");
        System.out.println(languagePreference);
        if(languagePreference.equals("english")) {
            Locale locale = new Locale("en");
            Configuration config = new Configuration();
            config.locale = locale;
            Resources res = getBaseContext().getResources();
            res.updateConfiguration(config, res.getDisplayMetrics());
        }
        else {
            Locale locale = new Locale("no");
            Configuration config = new Configuration();
            config.locale = locale;
            Resources res = getBaseContext().getResources();
            res.updateConfiguration(config, res.getDisplayMetrics());
        }
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        System.out.println("Language: " + getBaseContext().getResources().getConfiguration().getLocales().toLanguageTags());
    }

    @Override
    protected void onResume() {
        SharedPreferences spref = PreferenceManager.getDefaultSharedPreferences(this);
        String languagePreference = spref.getString(SettingsActivity.KEY_LANGUAGE_PREF, "english");
        System.out.println(languagePreference);
        if(languagePreference.equals("english")) {
            Locale locale = new Locale("en");
            Configuration config = new Configuration();
            config.locale = locale;
            Resources res = getBaseContext().getResources();
            res.updateConfiguration(config, res.getDisplayMetrics());
        }
        else {
            Locale locale = new Locale("no");
            Configuration config = new Configuration();
            config.locale = locale;
            Resources res = getBaseContext().getResources();
            res.updateConfiguration(config, res.getDisplayMetrics());
        }
        System.out.println("(Resume) Language: " + getBaseContext().getResources().getConfiguration().getLocales().toLanguageTags());
        setContentView(R.layout.activity_main);
        super.onResume();
    }

    public void generate_game(View v){
        String difficulty = "";
        switch(v.getId()) {
            case R.id.easy_button:
                difficulty = "easy";
                break;

            case R.id.medium_button:
                difficulty = "medium";
                break;

            case R.id.hard_button:
                difficulty = "hard";
                break;
            case R.id.add_new_game_button:
                difficulty = "new_game";
                break;
            case R.id.solved_board_btn:
                difficulty = "solved";
                break;
        }
        Intent intent = new Intent(getApplicationContext(), SudokuActivity.class);
        intent.putExtra(GAME_DIFFICULTY, difficulty);
        startActivity(intent);
    }
    public void open_tutorial(View v){
        Intent intent = new Intent(getApplicationContext(), TutorialActivity.class);
        startActivity(intent);

    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) {
        MenuInflater inflater = getMenuInflater();
        inflater.inflate(R.menu.menu_main, menu);
        return true;
    }

    @Override
    public boolean onOptionsItemSelected(MenuItem item) {
        if(item.getItemId() == R.id.action_settings){
            startActivity(new Intent(getApplicationContext(), SettingsActivity.class));
        }
        return true;
    }
}