import { Cli, MsgType } from '@natmri/bilicli-napi'
import { AttentionChangeMsg, startListen, WatchedChangeMsg } from 'blive-message-listener'



export class App {
  private cli!: Cli

  constructor(readonly roomId: number) {
    this.cli = new Cli(this.roomId)
    
    startListen(this.roomId, {
      onAttentionChange: ({ body }) => this.handleAttentionChange(body),
      onWatchedChange: ({ body }) => this.handleWatchedChange(body),
      onLiveStart: () => this.cli.sendLiveChange(true),
      onLiveEnd: () => this.cli.sendLiveChange(false),
      onIncomeDanmu: (msg) => {
        this.cli.sendMsg(MsgType.Danmu, JSON.stringify(msg.body))
      },
      onIncomeSuperChat: (msg) => {
        this.cli.sendMsg(MsgType.SuperChat, JSON.stringify({
          ...msg.body,
          timestamp: msg.timestamp,
        }))
      },
      onGift: (msg) => {
        this.cli.sendMsg(MsgType.Gift, JSON.stringify({
          ...msg.body,
          timestamp: msg.timestamp,
        }))
      },
      onGuardBuy: (msg) => {
        this.cli.sendMsg(MsgType.GuardBuy, JSON.stringify({
          ...msg.body,
          timestamp: msg.timestamp,
        }))
      },
      onUserAction: (msg) => {
        this.cli.sendMsg(MsgType.UserAction, JSON.stringify({
          ...msg.body,
          timestamp: msg.timestamp,
        }))
      }
    })
  }

  async run() {
    await this.cli.run()
  }

  private handleAttentionChange(body: AttentionChangeMsg) {
    this.cli.sendAttentionChange(body.attention)
  }

  private handleWatchedChange(body: WatchedChangeMsg) {
    this.cli.sendWatcherChange(body.num)
  }
}
