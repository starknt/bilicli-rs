import { AppState, Cli } from '@natmri/bilicli-napi'
import { AttentionChangeMsg, DanmuMsg, GiftMsg, GuardBuyMsg, Message, startListen, SuperChatMsg, UserActionMsg, WatchedChangeMsg } from 'blive-message-listener'



export class App {
  private cli!: Cli

  constructor(readonly roomId: number) {
    this.cli = new Cli(this.roomId)
    
    startListen(this.roomId, {
      onAttentionChange: ({ body }) => this.handleAttentionChange(body),
      onWatchedChange: ({ body }) => this.handleWatchedChange(body),
      onLiveStart: () => this.handleLiveStart(),
      onLiveEnd: () => this.handleLiveEnd(),
      onIncomeDanmu: (msg) => this.handleIncomeDanmu(msg),
      onIncomeSuperChat: (msg) => this.handleIncomeSuperChat(msg),
      onGift: (msg) => this.handleGift(msg),
      onGuardBuy: (msg) => this.handleGuardBuy(msg),
      onUserAction: (msg) => this.handleUserAction(msg)
    })
  }

  run() {
    this.cli.run()
  }

  private handleAttentionChange(body: AttentionChangeMsg) {
    this.cli.sendAttentionChange(body.attention)
  }

  private handleWatchedChange(body: WatchedChangeMsg) {
    console.log({body})
    this.cli.sendWatcherChange(body.text_small)
  }

  private handleLiveStart() {
    this.cli.sendLiveChange(true)
  }

  private handleLiveEnd() {
    this.cli.sendLiveChange(false)
  }

  private handleIncomeDanmu(msg: Message<DanmuMsg>) {}

  private handleIncomeSuperChat(msg: Message<SuperChatMsg>) {}
  
  private handleGift(msg: Message<GiftMsg>) {}

  private handleGuardBuy(msg: Message<GuardBuyMsg>) {}

  private handleUserAction(msg: Message<UserActionMsg>) {}
}
