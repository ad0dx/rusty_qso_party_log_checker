# rusty_qso_party_log_checker
A log checker for Amateur Radio Qso Party contest cabrillo files, written in Rust

This program checks and scores correctly formatted Cabrillo Log File from Amateur Radio State Qso Parties (contests).

The program is designed to be a generic qso party log checker.  It is based on the Kansas and Missouri qso parties.  If your qso party needs a feature not included, contact Ron ad0dx@yahoo.com or fork the repository and add the feature yourself.

Why Rust?  Mainly because I wanted to learn Rust.

Previously I wrote a C++ program for checking state Qso Parties and this Rust version will be very similiar.

The main features are:
- command line program (no gui interface)
- driven by text configuration files
- county names and abbreviations are provided in text files (so the program is not tied to any particular qso party)
- the program is designed to be support a generic qso party (special logic may need to be added for specific qso party bonus points)
- strong emphasis on cross checking qso's between logs
- when conflicts exist between logs, the instate log is taken as being correct most of the time
- has the ability to generate out of state logs based on the qso's in the in state logs
- log files are read in from disk in parallel and the first pass of log checking occurs on separate threads
- each log file generates a .txt file showing valid and invalid qso's along with multipliers
- a category definition file can be specified to place stations into contest categories

New Features not found in the previous C++ QsoPartyLogChecker program:
- addition of the x-qso feature which allows a qso to be marked as invalid in one log but valid in another
   - suppose that W1ABC works K6DEF but W1ABC logs the QSO as K6DER
   - if K6DER does not submit a log for the contest, then W1ABC gets credit for the QSO
   - K6DEF submits a logfile, but the qso is not found in the W1ABC log so K6DEF does not get credit for the qso
- allow multiple county abbreviations to be provided to allow for typo's in submitted logs
   - for example, JOH for Johnson County could also allow JON as an alternative, as long as JON is not another county abbreviation

