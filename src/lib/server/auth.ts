// import { db } from "./db";
import { randomBytes } from "crypto";
import bcrypt from "bcrypt";

const SESSION_TTL_MS = 1000 * 60 * 60 * 24 * 7; // 7 days

export type User = {
    id: string;
    email: string;
};

export async function authenticate(
    email: string,
    password: string,
): Promise<User | null> {
    const user: any = null; // await db.user.findUnique({ where: { email } });
    if (!user) return null;

    const ok = await bcrypt.compare(password, user.password_hash);
    if (!ok) return null;

    return { id: user.id, email: user.email };
}

export async function createSession(userId: string) {
    const id = randomBytes(32).toString("hex");
    const expiresAt = new Date(Date.now() + SESSION_TTL_MS);

    /*await db.session.create({
        data: {
            id,
            user_id: userId,
            expires_at: expiresAt,
        },
    });*/
    return { id, expiresAt };
}

export async function verifySession(sessionId: string): Promise<User | null> {
    const session: any = null; /* await db.session.findUnique({
        where: { id: sessionId },
        include: { user: true },
    });*/

    if (!session) return null;
    if (session.expires_at < new Date()) {
        // await db.session.delete({ where: { id: sessionId } });
        return null;
    }

    return {
        id: session.user.id,
        email: session.user.email,
    };
}

export async function destroySession(sessionId: string) {
    // await db.session.delete({ where: { id: sessionId } });
}
