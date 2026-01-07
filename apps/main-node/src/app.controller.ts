import { Controller, Get } from '@nestjs/common';
import { AppService } from './app.service';
import { MessagePattern, Payload } from '@nestjs/microservices';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) {}

  @MessagePattern('ping')
  handlePing(@Payload() data: string) {
    console.log(`[NestJS] Received: ${data}`);
    return `Pong from NestJS! (You sent: ${data})`;
  }
}
