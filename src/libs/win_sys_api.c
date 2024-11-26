#include <stdio.h>
#include <windows.h>
#include <tlhelp32.h>
#include <string.h>

DWORD get_explorer_pid()
{
    HANDLE snapshot;
    PROCESSENTRY32 pe32;
    DWORD exporer_pid = 0;

    snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if (snapshot == INVALID_HANDLE_VALUE)
    {
        return 0;
    }

    pe32.dwSize = sizeof(PROCESSENTRY32);

    if (!Process32First(snapshot, &pe32))
    {
        CloseHandle(snapshot);
        return 0;
    }

    do
    {
        if (strcmp(pe32.szExeFile, "explorer.exe") == 0)
        {
            exporer_pid = pe32.th32ProcessID;
            break;
        }
    } while (Process32Next(snapshot, &pe32));

    CloseHandle(snapshot);
    return exporer_pid;
}

void restart_explorer()
{
    DWORD explorer_pid = get_explorer_pid();
    if (explorer_pid == 0)
    {
        printf("Explorer not found\n");
        return;
    }

    HANDLE hExplorer = OpenProcess(PROCESS_TERMINATE, FALSE, explorer_pid);
    if (hExplorer == NULL)
    {
        printf("Failed to open explorer process\n");
        return;
    }

    if (!TerminateProcess(hExplorer, 0))
    {
        printf("Failed to terminate explorer process\n");
        CloseHandle(hExplorer);
        return;
    }

    CloseHandle(hExplorer);

    STARTUPINFO si = {sizeof(si)};
    PROCESS_INFORMATION pi;
    if (!CreateProcess(NULL, "explorer.exe", NULL, NULL, FALSE, 0, NULL, NULL, &si, &pi))
    {
        printf("Failed to start explorer process\n");
        return;
    }

    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);
}

void debug_print(const char *msg)
{
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    if (hConsole != INVALID_HANDLE_VALUE)
    {
        DWORD written;
        WriteConsoleA(hConsole, msg, strlen(msg), &written, NULL);
        WriteConsoleA(hConsole, "\n", 1, &written, NULL);
    }
}

__declspec(dllexport) void *list_all_pids()
{
    HANDLE snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if (snapshot == INVALID_HANDLE_VALUE)
    {
        return NULL;
    }

    PROCESSENTRY32 pe32;
    pe32.dwSize = sizeof(PROCESSENTRY32);

    if (!Process32First(snapshot, &pe32))
    {
        CloseHandle(snapshot);
        return NULL;
    }

    size_t pid_capacity = 10;
    size_t pid_count = 0;
    DWORD *pids = (DWORD *)malloc(pid_capacity * sizeof(DWORD));
    if (pids == NULL)
    {
        CloseHandle(snapshot);
        return NULL;
    }

    do
    {
        printf("PID: %d NAME: %s \n", pe32.th32ProcessID, pe32.szExeFile);
        fflush(stdout);
        Sleep(1);
        if (pid_count >= pid_capacity)
        {
            pid_capacity *= 2;
            DWORD *temp = (DWORD *)realloc(pids, pid_capacity * sizeof(DWORD));
            if (temp == NULL)
            {
                free(pids);
                CloseHandle(snapshot);
                return NULL;
            }
            pids = temp;
        }
        pids[pid_count++] = pe32.th32ProcessID;
    } while (Process32Next(snapshot, &pe32));

    pids[pid_count] = 0;
    CloseHandle(snapshot);
    return pids;
}

__declspec(dllexport) void free_pids(DWORD *pids)
{
    free(pids);
}