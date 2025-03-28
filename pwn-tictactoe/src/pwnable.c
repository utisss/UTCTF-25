#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <string.h>

void get_flag();

int main() {
    int board[] = {0, 0, 0, 0, 0, 0, 0, 0, 0};
    char chars[] = {' ', 'x', 'o'};
    int cpuwin = 0;
    int playerwin = 0;

    printf("Choose x or o: ");
    char buf[2];
    gets(buf);
    
    if (buf[0] == 'x') {
        chars[1] = 'o';
        chars[2] = 'x';
    } else if (buf[0] == 'o') {
    } else {
        printf("Unknown option\n");
        exit(0);
    }

    if (1) {
        // turn 1
        board[0] = 1;

        printf("Current board state: \n");
        printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
        
        // turn 2
        int t2move;
        while (1) {
            printf("Enter 1-9 to select a spot: ");
            char buf[2];
            gets(buf);
            if (board[buf[0] - '1'] != 0) {
                printf("Invalid spot!\n");
            } else {
                board[buf[0] - '1'] = 2;
                t2move = buf[0] - '1';
                break;
            }
        }

        printf("Current board state: \n");
        printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
        
        // turn 3
        if (t2move <= 2) board[3] = 1;
        else if (t2move >= 7) board[2] = 1;
        else if (t2move == 5) board[4] = 1;
        else board[1] = 1;

        printf("Current board state: \n");
        printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
        
        // turn 4
        int t4move;
        while (1) {
            printf("Enter 1-9 to select a spot: ");
            char buf[2];
            gets(buf);
            if (board[buf[0] - '1'] != 0) {
                printf("Invalid spot!\n");
            } else {
                board[buf[0] - '1'] = 2;
                t4move = buf[0] - '1';
                break;
            }
        }

        printf("Current board state: \n");
        printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
            

        // turn 5
        if (t2move <= 2) {
            if (t4move != 6) {
                board[6] = 1;
                cpuwin = 1;
            }
        }
        else if (t2move >= 7) {
            if (t4move != 1) {
                board[1] = 1;
                cpuwin = 1;
            }
        }
        else if (t2move == 5) {
            if (t4move != 8) {
                board[8] = 1;
                cpuwin = 1;
            }
        }
        else {
            if (t4move != 2) {
                board[2] = 1;
                cpuwin = 1;
            }
        }
        if (cpuwin) {
            printf("Current board state: \n");
            printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
            printf("CPU wins\n");
            exit(0);
        }      
        
        if (t2move <= 3 || t2move == 6 || t2move == 7) {
            board[4] = 1;
        }
        else if (t2move == 5) {
            board[2] = 1;
        }
        else {
            board[6] = 1;
        }

        printf("Current board state: \n");
        printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
        
        // turn 6
        int t6move;
        while (1) {
            printf("Enter 1-9 to select a spot: ");
            char buf[2];
            gets(buf);
            if (board[buf[0] - '1'] != 0) {
                printf("Invalid spot!\n");
            } else {
                board[buf[0] - '1'] = 2;
                t6move = buf[0] - '1';
                break;
            }
        }

        printf("Current board state: \n");
        printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
         

        // turn 7
        if (t2move == 1 || t2move == 2) {
            if (t6move == 5) {
                board[8] = 1;
                cpuwin = 1;
            }
            else {
                board[5] = 1;
                cpuwin = 1;
            }
        }
        else if (t2move == 3 || t2move == 6) {
            if (t6move == 7) {
                board[8] = 1;
                cpuwin = 1;
            }
            else {
                board[7] = 1;
                cpuwin = 1;
            }
        }
        else if (t2move == 5) {
            if (t6move == 1) {
                board[6] = 1;
                cpuwin = 1;
            }
            else {
                board[1] = 1;
                cpuwin = 1;
            }
        }
        else if (t2move == 7) {
            if (t6move == 6) {
                board[8] = 1;
                cpuwin = 1;
            }
            else {
                board[6] = 1;
                cpuwin = 1;
            }
        }
        else if (t2move == 8) {
            if (t6move == 3) {
                board[4] = 1;
                cpuwin = 1;
            }
            else {
                board[3] = 1;
                cpuwin = 1;
            }
        }
        else {
            if (t6move == 3) {
                board[5] = 1;
            }
            else {
                board[3] = 1;
                cpuwin = 1;
            }
        }

        printf("Current board state: \n");
        printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
            

        if (cpuwin) {
            printf("CPU wins\n");
            exit(0);
        }  

        // turn 8
        int t8move;
        while (1) {
            printf("Enter 1-9 to select a spot: ");
            char buf[2];
            gets(buf);
            if (board[buf[0] - '1'] != 0) {
                printf("Invalid spot!\n");
            } else {
                board[buf[0] - '1'] = 2;
                t8move = buf[0] - '1';
                break;
            }
        }

        printf("Current board state: \n");
        printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
         

        // turn 9
        if (t8move == 7) {
            board[8] = 1;
        }
        else {
            board[7] = 1;
        }

        printf("Current board state: \n");
        printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
        
        if (cpuwin) {
            printf("CPU wins\n");
        }
        else if (playerwin) {
            printf("Player wins\n");
            get_flag();
        }
        else {
            printf("Tie\n");
        }
        exit(0);        
    }
    // else {
    //     // turn 1
    //     int t1move;
    //     while (true) {
    //         printf("Enter 1-9 to select a spot: ");
    //         char buf[3];
    //         gets(buf);
    //         if (board[buf[0] - '1'] != 0) {
    //             printf("Invalid spot!\n");
    //         } else {
    //             board[buf[0] - '1'] = 2;
    //             t1move = buf[0] - '1';
    //             break;
    //         }
    //     }

    //     printf("Current board state: \n");
    //     printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
        
    //     // turn 2
    //     if (t1move != 4) {
    //         board[4] = 2;
    //     }
    //     else {
    //         board[0] = 2;
    //     }

    //     printf("Current board state: \n");
    //     printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
        
    //     // turn 3
    //     int t3move;
    //     while (true) {
    //         printf("Enter 1-9 to select a spot: ");
    //         char buf[3];
    //         gets(buf);
    //         if (board[buf[0] - '1'] != 0) {
    //             printf("Invalid spot!\n");
    //         } else {
    //             board[buf[0] - '1'] = 2;
    //             t3move = buf[0] - '1';
    //             break;
    //         }
    //     }

    //     printf("Current board state: \n");
    //     printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
        
    //     // turn 4
    //     if (t1move == 0) {
    //         if (t3move == 1) {
    //             board[2] = 2;
    //         }
    //         else if (t3move == 2) {
    //             board[1] = 2;
    //         }
    //         else if (t3move == 3) {
    //             board[6] = 2;
    //         }
    //         else if (t3move == 5) {
    //             board[7] = 2;
    //         }
    //         else if (t3move == 6) {
    //             board[3] = 2;
    //         }
    //         else if (t3move == 7) {
    //             board[5] = 2;
    //         }
    //         else {
    //             board[1] = 2;
    //         }
    //     }
    //     else if (t1move == 1) {
    //         if (t3move == 0) {
    //             board[2] = 2;
    //         }
    //         else if (t3move == 2) {
    //             board[0] = 2;
    //         }
    //         else if (t3move == 3) {
    //             board[2] = 2;
    //         }
    //         else if (t3move == 5) {
    //             board[0] = 2;
    //         }
    //         else if (t3move == 6) {
    //             board[3] = 2;
    //         }
    //         else if (t3move == 7) {
    //             board[5] = 2;
    //         }
    //         else {
    //             board[1] = 2;
    //         }
    //     }
    // }

    /*int[] playermoves = {-1, -1, -1, -1, -1};
    int playernum = 0;

    while (!over) {
        if (turn) {
            while (true) {
                printf("Enter 1-9 to select a spot: ");
                char buf[3];
                gets(buf);
                if (board[buf[0] - '1'] != 0) {
                    printf("Invalid spot!\n");
                } else {
                    board[buf[0] - '1'] = 2 - x;
                    playermoves[playernum++] = buf[0] - '1';
                    break;
                }
            }
            
        } else {
            printf("CPU is making a move\n");

            switch (tnum) {
                case 0:
                    board[0] = 2 - x;
                    break;
                case 1:

                case 2:

            }


            // try to win
            // bool placed = false;
            // if (board[0] == 2 - x) {
            //     if (board[1] == 2 - x) {
            //         if (board[2] == 0) {
            //             board[2] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[2] == 2 - x) {
            //         if (board[1] == 0) {
            //             board[1] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[3] == 2 - x) {
            //         if (board[6] == 0) {
            //             board[6] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[4] == 2 - x) {
            //         if (board[8] == 0) {
            //             board[8] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[6] == 2 - x) {
            //         if (board[3] == 0) {
            //             board[3] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[8] == 2 - x) {
            //         if (board[4] == 0) {
            //             board[4] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            // }
            // else if (board[1] == 2 - x) {
            //     if (board[2] == 2 - x) {
            //         if (board[0] == 0) {
            //             board[0] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[4] == 2 - x) {
            //         if (board[7] == 0) {
            //             board[7] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[7] == 2 - x) {
            //         if (board[4] == 0) {
            //             board[4] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            // }
            // else if (board[2] == 2 - x) {
            //     if (board[4] == 2 - x) {
            //         if (board[6] == 0) {
            //             board[6] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[5] == 2 - x) {
            //         if (board[8] == 0) {
            //             board[8] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[6] == 2 - x) {
            //         if (board[4] == 0) {
            //             board[4] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[8] == 2 - x) {
            //         if (board[5] == 0) {
            //             board[5] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            // }
            // else if (board[3] == 2 - x) {
            //     if (board[4] == 2 - x) {
            //         if (board[5] == 0) {
            //             board[5] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[5] == 2 - x) {
            //         if (board[4] == 0) {
            //             board[4] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[6] == 2 - x) {
            //         if (board[0] == 0) {
            //             board[0] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            // }
            // else if (board[4] == 2 - x) {
            //     if (board[5] == 2 - x) {
            //         if (board[3] == 0) {
            //             board[3] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[6] == 2 - x) {
            //         if (board[2] == 0) {
            //             board[2] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[7] == 2 - x) {
            //         if (board[1] == 0) {
            //             board[1] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[8] == 2 - x) {
            //         if (board[0] == 0) {
            //             board[0] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            // }
            // else if (board[5] == 2 - x) {
            //     if (board[8] == 2 - x) {
            //         if (board[2] == 0) {
            //             board[2] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            // }
            // else if (board[6] == 2 - x) {
            //     if (board[7] == 2 - x) {
            //         if (board[8] == 0) {
            //             board[8] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            //     else if (board[8] == 2 - x) {
            //         if (board[7] == 0) {
            //             board[7] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            // }
            // else if (board[7] == 2 - x) {
            //     if (board[8] == 2 - x) {
            //         if (board[6] == 0) {
            //             board[6] = 2 - x;
            //             placed = true;
            //             over = true;
            //             cpuwin = true;
            //         }
            //     }
            // }
            // // try to block
            // if (!placed) {
            //     if (board[0] == 1 + x) {
            //         if (board[1] == 1 + x) {
            //             if (board[2] == 0) {
            //                 board[2] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[2] == 1 + x) {
            //             if (board[1] == 0) {
            //                 board[1] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[3] == 1 + x) {
            //             if (board[6] == 0) {
            //                 board[6] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[4] == 1 + x) {
            //             if (board[8] == 0) {
            //                 board[8] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[6] == 1 + x) {
            //             if (board[3] == 0) {
            //                 board[3] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[8] == 1 + x) {
            //             if (board[4] == 0) {
            //                 board[4] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //     }
            //     else if (board[1] == 1 + x) {
            //         if (board[2] == 1 + x) {
            //             if (board[0] == 0) {
            //                 board[0] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[4] == 1 + x) {
            //             if (board[7] == 0) {
            //                 board[7] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[7] == 1 + x) {
            //             if (board[4] == 0) {
            //                 board[4] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //     }
            //     else if (board[2] == 1 + x) {
            //         if (board[4] == 1 + x) {
            //             if (board[6] == 0) {
            //                 board[6] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[5] == 1 + x) {
            //             if (board[8] == 0) {
            //                 board[8] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[6] == 1 + x) {
            //             if (board[4] == 0) {
            //                 board[4] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[8] == 1 + x) {
            //             if (board[5] == 0) {
            //                 board[5] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //     }
            //     else if (board[3] == 1 + x) {
            //         if (board[4] == 1 + x) {
            //             if (board[5] == 0) {
            //                 board[5] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[5] == 1 + x) {
            //             if (board[4] == 0) {
            //                 board[4] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[6] == 1 + x) {
            //             if (board[0] == 0) {
            //                 board[0] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //     }
            //     else if (board[4] == 1 + x) {
            //         if (board[5] == 1 + x) {
            //             if (board[3] == 0) {
            //                 board[3] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[6] == 1 + x) {
            //             if (board[2] == 0) {
            //                 board[2] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[7] == 1 + x) {
            //             if (board[1] == 0) {
            //                 board[1] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[8] == 1 + x) {
            //             if (board[0] == 0) {
            //                 board[0] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //     }
            //     else if (board[5] == 1 + x) {
            //         if (board[8] == 1 + x) {
            //             if (board[2] == 0) {
            //                 board[2] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //     }
            //     else if (board[6] == 1 + x) {
            //         if (board[7] == 1 + x) {
            //             if (board[8] == 0) {
            //                 board[8] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //         else if (board[8] == 1 + x) {
            //             if (board[7] == 0) {
            //                 board[7] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //     }
            //     else if (board[7] == 1 + x) {
            //         if (board[8] == 1 + x) {
            //             if (board[6] == 0) {
            //                 board[6] = 2 - x;
            //                 placed = true;
            //                 over = true;
            //                 cpuwin = true;
            //             }
            //         }
            //     }
            // }
            // // try to fork


        }
        printf("Current board state: \n");
        printf("%c%c%c\n%c%c%c\n%c%c%c\n", chars[board[0]], chars[board[1]], chars[board[2]], chars[board[3]], chars[board[4]], chars[board[5]], chars[board[6]], chars[board[7]], chars[board[8]]);
        x = 1 - x;
        turn = !turn;
        tnum++;
    }
        */
}

void get_flag() {
    char*  args[2] = {"/bin/sh", NULL};
    execve(args[0], args, NULL);
}
