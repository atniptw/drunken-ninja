package com.hackathon.homebrewnavigator.recipes;

import android.support.test.filters.FlakyTest;
import android.support.v4.widget.DrawerLayout;
import android.test.ActivityInstrumentationTestCase2;
import android.view.Gravity;

import com.hackathon.homebrewnavigator.MainActivity;
import com.hackathon.homebrewnavigator.R;
import com.robotium.solo.Solo;

import junit.framework.Assert;

public class StartNewBrewFromRecipe extends ActivityInstrumentationTestCase2<MainActivity> {

    private Solo solo;

    public StartNewBrewFromRecipe() {
        super(MainActivity.class);
    }

    public void setUp() throws Exception {
        solo = new Solo(getInstrumentation(), getActivity());
        solo.assertCurrentActivity("wrong activity", MainActivity.class);
    }

    @Override
    public void tearDown() throws Exception {
        solo.finishOpenedActivities();
    }

    public void openCompatNavigationDrawer() {
        getInstrumentation().runOnMainSync(new Runnable() {
            @Override
            public void run() {
                ((DrawerLayout) solo.getView(R.id.drawer_layout)).openDrawer(Gravity.LEFT);
            }
        });
    }

    @FlakyTest
    public void testStartNewBrewFromExistingRecipe() {
        openCompatNavigationDrawer();
        solo.waitForView(DrawerLayout.class);

        Assert.assertTrue("Navigation drawer should have Recipes option", solo.searchText(solo.getString(R.string.recipes)));

        solo.takeScreenshot("testStartNewBrewFromExistingRecipe");
    }
}
