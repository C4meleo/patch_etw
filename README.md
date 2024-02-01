# patch_etw
Patch to bypass the ETW function

To detect potential threats, EDRs can collect information on a Windows system. To recover all
this information, the EDRs will use ETW (= Event Tracing for Windows). ETW is an event tracking mechanism
integrated into Windows. This is a function for tracing and logging events triggered by applications, in mode
user and kernel mode drivers, in a log file. Events can be consumed in real time or from
of a log file. 
In this repo you will find a patch to bypass the ETW function. This will avoid the
detection by the EDRs of our programs, for example.
