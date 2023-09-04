import { JSCompletionTrie } from 'wlang-web'

export const WORKOUTS: Record<string, string> = {
    simple: `# Bench Press
225 x5 
        `,
    complex: '',
}

const seed_exercises: string[] = [
    'bench press',
    'back squat',
    'deadlift',
    'front squat',
]

export const COMPLETION = new JSCompletionTrie()
seed_exercises.forEach(e => COMPLETION.addExercise(e))
