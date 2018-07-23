// copied form emscripten emrun.py

// Lift the default max 20 workers limit to something higher to avoid hangs when page needs to spawn a lot of threads.
user_pref("dom.workers.maxPerDomain", 100);
// Always allow opening popups
user_pref("browser.popups.showPopupBlocker", false);
user_pref("dom.disable_open_during_load", false);
// Don't ask user if he wants to set Firefox as the default system browser
user_pref("browser.shell.checkDefaultBrowser", false);
user_pref("browser.shell.skipDefaultBrowserCheck", true);
// If automated runs crash, don't resume old tabs on the next run or show safe mode dialogs or anything else extra.
user_pref("browser.sessionstore.resume_from_crash", false);
user_pref("services.sync.prefs.sync.browser.sessionstore.restore_on_demand", false);
user_pref("browser.sessionstore.restore_on_demand", false);
user_pref("browser.sessionstore.max_resumed_crashes", -1);
user_pref("toolkip.startup.max_resumed_crashes", -1);
// Don't show the slow script dialog popup
user_pref("dom.max_script_run_time", 0);
user_pref("dom.max_chrome_script_run_time", 0);
// Don't open a home page at startup
user_pref("startup.homepage_override_url", "about:blank");
user_pref("startup.homepage_welcome_url", "about:blank");
user_pref("browser.startup.homepage", "about:blank");
// Don't try to perform browser (auto)update on the background
user_pref("app.update.auto", false);
user_pref("app.update.enabled", false);
user_pref("app.update.silent", false);
user_pref("app.update.mode", 0);
user_pref("app.update.service.enabled", false);
// Don't check compatibility with add-ons, or (auto)update them
user_pref("extensions.lastAppVersion", '');
user_pref("plugins.hide_infobar_for_outdated_plugin", true);
user_pref("plugins.update.url", '');
// Disable health reporter
user_pref("datareporting.healthreport.service.enabled", false);
// Disable crash reporter
user_pref("toolkit.crashreporter.enabled", false);
// Don't show WhatsNew on first run after every update
user_pref("browser.startup.homepage_override.mstone","ignore");
// Don't show 'know your rights' and a bunch of other nag windows at startup
user_pref("browser.rights.3.shown", true);
user_pref('devtools.devedition.promo.shown', true);
user_pref('extensions.shownSelectionUI', true);
user_pref('browser.newtabpage.introShown', true);
user_pref('browser.download.panel.shown', true);
user_pref('browser.customizemode.tip0.shown', true);
user_pref("browser.toolbarbuttons.introduced.pocket-button", true);
// Start in private browsing mode to not cache anything to disk (everything will be wiped anyway after this run)
user_pref("browser.privatebrowsing.autostart", true);
// Don't ask the user if he wants to close the browser when there are multiple tabs.
user_pref("browser.tabs.warnOnClose", false);
// Allow the launched script window to close itself, so that we don't need to kill the browser process in order to move on.
user_pref("dom.allow_scripts_to_close_windows", true);
// Set various update timers to a large value in the future in order to not
// trigger a large mass of update HTTP traffic on each Firefox run on the clean profile.
// "01/01/2100" is 4102437600 as seconds since Unix epoch.
user_pref("app.update.lastUpdateTime.addon-background-update-timer", 4102437600);
user_pref("app.update.lastUpdateTime.background-update-timer", 4102437600);
user_pref("app.update.lastUpdateTime.blocklist-background-update-timer", 4102437600);
user_pref("app.update.lastUpdateTime.browser-cleanup-thumbnails", 4102437600);
user_pref("app.update.lastUpdateTime.experiments-update-timer", 4102437600);
user_pref("app.update.lastUpdateTime.search-engine-update-timer", 4102437600);
user_pref("app.update.lastUpdateTime.xpi-signature-verification", 4102437600);
user_pref("extensions.getAddons.cache.lastUpdate", 4102437600);
user_pref("media.gmp-eme-adobe.lastUpdate", 4102437600);
user_pref("media.gmp-gmpopenh264.lastUpdate", 4102437600);
user_pref("datareporting.healthreport.nextDataSubmissionTime", 4102437600439);
// Detect directly when executing if asm.js does not validate by throwing an error.
user_pref("javascript.options.throw_on_asmjs_validation_failure", true);
// Sending Firefox Health Report Telemetry data is not desirable, since these are automated runs.
user_pref("datareporting.healthreport.uploadEnabled", false);
user_pref("datareporting.healthreport.service.enabled", false);
user_pref("datareporting.healthreport.service.firstRun", false);
user_pref("toolkit.telemetry.enabled", false);
user_pref("toolkit.telemetry.unified", false);
user_pref("datareporting.policy.dataSubmissionEnabled", false);
user_pref("datareporting.policy.dataSubmissionPolicyBypassNotification", true);
// Allow window.dump() to print directly to console
user_pref("browser.dom.window.dump.enabled", true);
// Disable background add-ons related update & information check pings
user_pref("extensions.update.enabled", false);
user_pref("extensions.getAddons.cache.enabled", false);
// Enable wasm
user_pref("javascript.options.wasm", true);
// Enable SharedArrayBuffer (this profile is for a testing environment, so Spectre/Meltdown don't apply)
user_pref("javascript.options.shared_memory", true);

