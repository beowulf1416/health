export class User {

    constructor(
        private email: string
    ) {}

    get is_logged_in(): boolean {
        return this.email != "";
    }

    get email_address(): string {
        return this.email;
    }
}
