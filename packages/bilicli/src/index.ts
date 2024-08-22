#!/bin/env node

import { cac } from 'cac'
import { version } from '../package.json'
import { App, AppOptions, EditOptions } from './app'

const cli = cac('bilicli')

cli
  .command('<room_id>', '输入房间号，打开直播间控制台')
  .option('-c, --cookie <cookie>', '将你在B站登录的cookie粘贴到这里')
  .option('-u, --uid <uid>', '你的B站UID')
  .option('--config <config>', '配置文件路径')
  .action(async (roomId: string, options: AppOptions) => {
    const app = new App(roomId, options)
    await app.run()
    process.exit(0)
  })

cli
  .command('edit', 'Open editor to edit config')
  .option('-c, --config <config>', 'Custom config file path', { default: 'biliclirc' })
  .action((options: EditOptions) => {
    // TODO: open editor to edit config
    console.log(options)
  })

cli.help()
cli.version(version)

cli.parse()

