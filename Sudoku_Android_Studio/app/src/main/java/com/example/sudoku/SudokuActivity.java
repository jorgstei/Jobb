package com.example.sudoku;

import android.content.Context;
import android.content.Intent;
import android.os.Bundle;
import android.view.Gravity;
import android.view.Menu;
import android.view.MenuInflater;
import android.view.MenuItem;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ArrayAdapter;
import android.widget.Button;
import android.widget.EditText;
import android.widget.GridView;
import android.widget.LinearLayout;
import android.widget.PopupWindow;
import android.widget.Spinner;
import android.widget.TextView;
import android.widget.Toast;

import androidx.appcompat.app.AppCompatActivity;

import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Random;
import java.util.concurrent.ExecutionException;

import static com.example.sudoku.MainActivity.GAME_DIFFICULTY;

public class SudokuActivity extends AppCompatActivity {
    CostumAdapter adap;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_sudoku);
        String difficulty = getIntent().getExtras().getString(GAME_DIFFICULTY);
        TextView tw1 = findViewById(R.id.tw1);
        tw1.setText(difficulty);

        File[] files = getFilesDir().listFiles();
        for (int i = 0; i < files.length; i++) {
            System.out.println(files[i].getName());
        }
        if(difficulty.equals("new_game")){
            start_create_new_game();
        }
        else{
            start_game(difficulty);
        }

    }
    public void start_create_new_game(){
        int[] initialBoard = {0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0};
        GridView grid = (GridView) findViewById(R.id.board);
        LinearLayout layout = (LinearLayout) findViewById(R.id.header_layout);
        layout.removeAllViewsInLayout();
        //Initialize new header layout
        Spinner dropdown = new Spinner(getApplicationContext());
        dropdown.setId(R.id.difficulty_dropdown);
        dropdown.setLayoutParams(new LinearLayout.LayoutParams(LinearLayout.LayoutParams.WRAP_CONTENT, LinearLayout.LayoutParams.WRAP_CONTENT));
        String[] difficulties = {"easy", "medium", "hard"};
        ArrayAdapter<String> dropdownAdapter = new ArrayAdapter<String>(this, android.R.layout.simple_spinner_dropdown_item, difficulties);
        dropdownAdapter.setDropDownViewResource(android.R.layout.simple_spinner_dropdown_item);
        dropdown.setAdapter(dropdownAdapter);
        Button btn = new Button(getApplicationContext());
        btn.setText("Publish game");
        btn.setId(R.id.new_game_button);
        btn.setGravity(Gravity.CENTER_HORIZONTAL);
        btn.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                String difficulty = ((Spinner) findViewById(R.id.difficulty_dropdown)).getSelectedItem().toString();
                create_new_board(adap.getValues(), difficulty);
                Intent intent = new Intent(getApplicationContext(), MainActivity.class);
                startActivity(intent);
            }
        });

        layout.addView(dropdown);
        layout.addView(btn);
        adap = new CostumAdapter(getApplicationContext(), initialBoard);
        grid.setAdapter(adap);
    }
    public void start_game(String difficulty) {
        if(difficulty.equals("solved")){
            GridView grid = (GridView) findViewById(R.id.board);
            int[] solution = {8, 4, 6, 9, 3, 7, 1, 5, 2, 3, 1, 9, 6, 2, 5, 8, 4, 7, 7, 5, 2, 1, 8, 4, 9, 6, 3, 2, 8, 5, 7, 1, 3, 6, 9, 4, 4, 6, 3, 8, 5, 9, 2, 7, 1, 9, 7, 1, 2, 4, 6, 3, 8, 5, 1, 2, 7, 5, 9, 8, 4, 3, 6, 6, 3, 8, 4, 7, 1, 5, 2, 9, 5, 9, 4, 3, 6, 2, 7, 1, 8};
            adap = new CostumAdapter(getApplicationContext(), solution);
            grid.setAdapter(adap);
        }
        else {
            File[] allFiles = getFilesDir().listFiles();
            ArrayList<File> eligableFiles = new ArrayList<>();
            for (int i = 0; i < allFiles.length; i++) {
                if (allFiles[i].getName().toLowerCase().charAt(0) == difficulty.toLowerCase().charAt(0)) {
                    eligableFiles.add(allFiles[i]);
                }
            }
            Random random = new Random();
            int chosenIndex = random.nextInt(eligableFiles.size());
            ArrayList<Integer> board = new ArrayList<>();
            try {
                FileInputStream fin = openFileInput(eligableFiles.get(chosenIndex).getName());
                int num;
                while ((num = fin.read()) != -1) {
                    board.add(num);
                }
                fin.close();
            } catch (Exception e) {
                e.printStackTrace();
            }
            if (board.size() >= 81) {
                board = new ArrayList<>(board.subList(0, 81));
            }
            int[] primitiveBoard = new int[board.size()];
            for (int i = 0; i < primitiveBoard.length; i++) {
                primitiveBoard[i] = board.get(i);
            }

            GridView grid = (GridView) findViewById(R.id.board);
            adap = new CostumAdapter(getApplicationContext(), primitiveBoard);
            grid.setAdapter(adap);
        }
    }

    public boolean create_new_board(int[] arr, String difficulty) {
        try {
            File[] files = getFilesDir().listFiles();
            int difficultyCounter = 0;
            for (int i = 0; i < files.length; i++) {
                if (files[i].getName().toLowerCase().charAt(0) == difficulty.toLowerCase().charAt(0)) {
                    difficultyCounter++;
                }
            }
            FileOutputStream fout = openFileOutput(difficulty.toLowerCase() + "" + difficultyCounter, Context.MODE_PRIVATE);
            for (int i = 0; i < arr.length; i++) {
                fout.write(arr[i]);
            }
            fout.close();
            return true;
        } catch (Exception e) {
            e.printStackTrace();
            return false;
        }
    }

    public void check_solution(View v) {
        System.out.println(adap.getValues().length);
        int[][] completedVals = convert1dTo2d(adap.getValues());
        //print2dArray(completedVals);
        if(correct_rows_and_columns(completedVals) && correct_squares(completedVals)){
            TextView tw = new TextView(this);
            tw.setText(getResources().getString(R.string.congratulations_string));
            tw.setGravity(Gravity.CENTER_HORIZONTAL);
            tw.setBackgroundColor(v.getResources().getColor(android.R.color.holo_green_light));
            tw.setTextSize(40);
            PopupWindow popup = new PopupWindow(tw, LinearLayout.LayoutParams.MATCH_PARENT, LinearLayout.LayoutParams.WRAP_CONTENT, true);
            popup.setOnDismissListener(new PopupWindow.OnDismissListener() {
                @Override
                public void onDismiss() {
                    Intent intent = new Intent(getApplicationContext(), MainActivity.class);
                    startActivity(intent);
                }
            });
            popup.showAtLocation(v, Gravity.CENTER, 10, 10);
        }
        else{
            Toast.makeText(getApplicationContext(), getResources().getString(R.string.incorrect_string), Toast.LENGTH_LONG).show();
        }
    }
    public void print2dArray(int[][] arr){
        for (int i = 0; i < arr.length; i++) {
            String s = "";
            for (int j = 0; j < arr[i].length; j++) {
                s += arr[i][j] + ", ";
            }
            System.out.println(s);
        }
    }

    public int[][] convert1dTo2d(int[] arr) {
        int[][] res = new int[9][9];
        //Row
        for (int i = 0; i < 9; i++) {
            //Elements in row
            for (int j = 0; j < 9; j++) {
                if (i == 0) {
                    res[i][j] = arr[1 * j];
                    //System.out.println(i + ", " + j + " gets val " + arr[1*j]);
                } else if (j == 0) {
                    res[i][j] = arr[i * 9];
                    //System.out.println(i + ", " + j + " gets val " + arr[i*9*1]);
                } else {
                    res[i][j] = arr[i * 9 + j];
                    //System.out.println(i + ", " + j + " gets val " + arr[i*9+j]);
                }
            }
        }
        return res;
    }

    public boolean correct_rows_and_columns(int[][] arr) {
        if ((arr.length * arr[0].length) != 81) {
            System.out.println("Must be 9x9 grid");
            return false;
        }
        for (int i = 0; i < 9; i++) {
            boolean[] oneThroughNineRow = {false, false, false, false, false, false, false, false, false};
            boolean[] oneThroughNineColumn = {false, false, false, false, false, false, false, false, false};
            for (int j = 0; j < 9; j++) {
                try {
                    oneThroughNineRow[arr[i][j] - 1] = true;
                    oneThroughNineColumn[arr[j][i] - 1] = true;
                } catch (IndexOutOfBoundsException e) {
                    //Only throws indexoutofbounds if value of a cell was 0, which means the user didnt input a value, ie uncompleted puzzle.
                    //e.printStackTrace();
                    return false;
                }
            }
            for (int k = 0; k < 9; k++) {
                if (!oneThroughNineRow[k]) {

                    return false;
                } else if (!oneThroughNineColumn[k]) {

                    return false;
                }
            }
        }
        return true;
    }

    public boolean correct_squares(int[][] arr) {
        if ((arr.length * arr[0].length) != 81) {
            return false;
        }
        // i=square column, j= square row, k=row, l=column
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                boolean[] oneThroughNineSquare = {false, false, false, false, false, false, false, false, false};

                for (int k = 0; k < 3; k++) {
                    for (int l = 0; l < 3; l++) {
                        try {
                            oneThroughNineSquare[arr[(3*i)+k][(3*j)+l] - 1] = true;

                        } catch (IndexOutOfBoundsException e) {
                            //Only throws indexoutofbounds if value of a cell was 0, which means the user didnt input a value, ie uncompleted puzzle.
                            //e.printStackTrace();
                            return false;
                        }
                    }
                }
                for (int h = 0; h < oneThroughNineSquare.length; h++) {
                    if(!oneThroughNineSquare[h]){
                        return false;
                    }
                }
            }

        }
        return true;
    }
    private boolean delete_file(String filename){
        File[] files = getFilesDir().listFiles();
        for (int i = 0; i < files.length; i++) {
            if(files[i].getName().equals(filename)){
                return files[i].delete();
            }
        }
        return false;
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
