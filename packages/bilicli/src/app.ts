import { Cli, MsgType } from '@natmri/bilicli-napi'
import { Message, MsgHandler, startListen } from 'blive-message-listener'

export interface AppOptions {
  cookie?: string
  config?: string
  uid?: string
}

export interface EditOptions {
  config: string
}

export class App {
  private readonly roomId: number
  private readonly cli: Cli

  constructor(roomId: string, private options: AppOptions) {
    this.roomId = parseInt(roomId)

    if (isNaN(this.roomId)) {
      throw new Error('房间号格式不正确')
    }

    this.cli = new Cli(this.roomId)

    const handler: MsgHandler = {
      onAttentionChange: ({ body }) => this.cli.sendAttentionChange(body.attention),
      onWatchedChange: ({ body }) => this.cli.sendWatcherChange(body.num),
      onLiveStart: () => this.cli.sendLiveChange(true),
      onLiveEnd: () => this.cli.sendLiveChange(false),
      onIncomeDanmu: (msg) => this.cli.sendMsg(MsgType.Danmu, JSON.stringify(this.mixTimestamp2Body(msg))),
      onIncomeSuperChat: (msg) =>  this.cli.sendMsg(MsgType.SuperChat, JSON.stringify(this.mixTimestamp2Body(msg))),
      onGift: (msg) => this.cli.sendMsg(MsgType.Gift, JSON.stringify(this.mixTimestamp2Body(msg))),
      onGuardBuy: (msg) => this.cli.sendMsg(MsgType.GuardBuy, JSON.stringify(this.mixTimestamp2Body(msg))),
      onUserAction: (msg) => this.cli.sendMsg(MsgType.UserAction, JSON.stringify(this.mixTimestamp2Body(msg))),
    }

    let uid: number | undefined
    if (this.options?.uid) {
      uid = parseInt(this.options.uid)
      if (isNaN(uid)) {
        throw new Error('UID 格式不正确')
      }
    }

    startListen(this.roomId, handler, {
      ws: {
        headers: {
          'Cookie': this.options?.cookie || "",
        },
        uid,
      },
    })
  }

  async run() {
    await this.cli.run()
  }

  private mixTimestamp2Body(msg: Message<any>) {
    return {
      ...msg.body,
      timestamp: msg?.timestamp || Date.now(),
    }
  }
}
