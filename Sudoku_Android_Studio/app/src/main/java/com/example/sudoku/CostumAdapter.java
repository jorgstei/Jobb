package com.example.sudoku;

import android.content.Context;
import android.graphics.Color;
import android.text.Editable;
import android.text.InputFilter;
import android.text.InputType;
import android.text.Layout;
import android.text.Spanned;
import android.text.TextWatcher;
import android.text.method.DigitsKeyListener;
import android.util.Log;
import android.view.Gravity;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.BaseAdapter;
import android.widget.EditText;
import android.widget.GridView;
import android.widget.ImageView;
import android.widget.LinearLayout;
import android.widget.TextView;

import java.util.ArrayList;

public class CostumAdapter extends BaseAdapter {
    Context ctx;
    int[] values;
    String[] hintValues = new String[81];
    LayoutInflater inflater;

    public CostumAdapter(Context appContext, int[] vals){
        System.out.println("CREATED COSTUMADAPTER");
        this.ctx = appContext;
        this.values = vals;
        this.inflater = (LayoutInflater.from(ctx));
    }
    @Override
    public int getCount(){
        return values.length;
    }
    @Override
    public Object getItem(int item){
        return null;
    }
    @Override
    public long getItemId(int i) {
        return 0;
    }

    public int[] getValues(){
        return this.values;
    }
    @Override
    public View getView(final int position, View view, ViewGroup viewGroup) {
        if(position >=81){
            return new TextView(ctx);
        }
        float scale = viewGroup.getResources().getDisplayMetrics().density;
        int dpAsPixels = (int) (50*scale + 0.5f);
        LinearLayout lin = new LinearLayout(ctx);
        lin.setOrientation(LinearLayout.VERTICAL);
        lin.setLayoutParams(new LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, dpAsPixels));
        lin.setBackgroundColor(viewGroup.getResources().getColor(android.R.color.white));
        lin.setWeightSum(4);

        LinearLayout.LayoutParams lp = new LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, 0, 1.0f);
        lp.setMargins(0,0,0,0);
        final EditText hintText = new EditText(ctx);
        hintText.setPadding(0,0,0,0);
        hintText.setLayoutParams(lp);
        if(hintValues[position] != null){
            hintText.setText(hintValues[position]);
        }
        else{
            hintText.setHint("1,2,3..");
        }
        hintText.setTextSize(10);
        hintText.setBackgroundColor(viewGroup.getResources().getColor(android.R.color.transparent));
        hintText.setGravity(Gravity.RIGHT);
        hintText.addTextChangedListener(new TextWatcher() {
            @Override
            public void beforeTextChanged(CharSequence charSequence, int i, int i1, int i2) {

            }

            @Override
            public void onTextChanged(CharSequence charSequence, int i, int i1, int i2) {
                if(charSequence.length()!=0){
                    hintValues[position] = charSequence.toString();
                }
                else{
                    hintValues[position] = null;
                }
            }

            @Override
            public void afterTextChanged(Editable editable) {

            }
        });

        LinearLayout.LayoutParams lp2 = new LinearLayout.LayoutParams(LinearLayout.LayoutParams.MATCH_PARENT, 0, 3.0f);
        EditText input = new EditText(ctx);
        input.setPadding(0,0,0,0);
        input.setLayoutParams(lp2);
        input.setTextSize(25);
        input.setBackgroundColor(viewGroup.getResources().getColor(android.R.color.transparent));
        if(values[position]!=0) {
            input.setText(values[position] + "");
        }

        input.setInputType(InputType.TYPE_CLASS_NUMBER);
        input.setKeyListener(DigitsKeyListener.getInstance("123456789"));
        input.setFilters(new InputFilter[]{new InputFilter.LengthFilter(1)});
        input.setGravity(Gravity.CENTER_HORIZONTAL);
        input.addTextChangedListener(new TextWatcher() {
            @Override
            public void beforeTextChanged(CharSequence charSequence, int i, int i1, int i2) {}

            @Override
            public void onTextChanged(CharSequence charSequence, int i, int i1, int i2) {

                if(charSequence.length()!=0){
                    int num = Integer.parseInt(charSequence.toString());
                    values[position] = num;
                }
                else{
                    values[position] = 0;
                }
            }

            @Override
            public void afterTextChanged(Editable editable) {}
        });

        lin.addView(hintText);
        lin.addView(input);
        return lin;
    }
}
