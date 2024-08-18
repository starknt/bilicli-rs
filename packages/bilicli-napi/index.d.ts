/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum AppState {
  Running = 0,
  Quitting = 1,
  Quit = 2
}
export declare class Cli {
  roomId: number
  constructor(roomId: number)
  get state(): AppState
  run(): void
  stop(): void
  sendAttentionChange(attention: number): void
  sendWatcherChange(watcher: string): void
  sendLiveChange(live: boolean): void
}
