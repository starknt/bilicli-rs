import { Cli, MsgType } from '@natmri/bilicli-napi'
import { AttentionChangeMsg, MsgHandler, startListen, WatchedChangeMsg } from 'blive-message-listener'

export interface AppOptions {
  cookie?: string
  config?: string
}

export interface EditOptions {
  config: string
}

export class App {
  private cli!: Cli

  constructor(readonly roomId: number, private options: AppOptions) {
    this.cli = new Cli(this.roomId)

    const handler: MsgHandler = {
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
      },
    }
    
    startListen(this.roomId, handler, {
      ws: {
        headers: {
          'Cookie': this.options?.cookie || '',
        }
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
