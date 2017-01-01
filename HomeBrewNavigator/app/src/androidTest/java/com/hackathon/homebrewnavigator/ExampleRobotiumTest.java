package com.hackathon.homebrewnavigator;

import android.test.ActivityInstrumentationTestCase2;

import com.robotium.solo.Solo;

public class ExampleRobotiumTest extends ActivityInstrumentationTestCase2<MainActivity> {

    private Solo solo;

    public ExampleRobotiumTest() {
        super(MainActivity.class);
    }

    public void setUp() throws Exception {
        solo = new Solo(getInstrumentation(), getActivity());
    }

    @Override
    public void tearDown() throws Exception {
        solo.finishOpenedActivities();
    }

    public void testActivityLaunches() {
        solo.assertCurrentActivity("wrong activity", MainActivity.class);
    }
}
